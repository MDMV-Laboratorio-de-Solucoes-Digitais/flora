//! Application state that holds connections, services, and configuration.

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use std::sync::Arc;

use flora_core::repositories::{
    PgChannelRepository, PgFileRepository, PgMembershipRepository, PgMessageRepository,
    PgNotificationRepository, PgOrganizationRepository, PgRoleRepository, PgSessionRepository,
    PgTaskRepository, PgUserRepository, PgWorkspaceRepository,
};
use flora_core::services::{
    ChannelService, FileService, MessageService, NotificationService, OrganizationService,
    RbacService, SessionService, TaskService, WorkspaceService,
};
use flora_core::{PgPool, config::Config};

use crate::realtime;

use super::oidc_transactions::OidcTransactionStore;

/// Application state shared across all requests.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Database connection pool.
    pub db_pool: Arc<PgPool>,
    /// Application configuration.
    pub config: Arc<Config>,
    /// RBAC service.
    pub rbac_service: Arc<RbacService>,
    /// Session service.
    pub session_service: Arc<SessionService>,
    /// Organization service.
    pub organization_service: Arc<OrganizationService>,
    /// Workspace service.
    pub workspace_service: Arc<WorkspaceService>,
    /// Channel service.
    pub channel_service: Arc<ChannelService>,
    /// Message service.
    pub message_service: Arc<MessageService>,
    /// Task service.
    pub task_service: Arc<TaskService>,
    /// File service.
    pub file_service: Arc<FileService>,
    /// Notification service.
    pub notification_service: Arc<NotificationService>,
    /// Search service (optional; disabled/unavailable in lean Seed profiles).
    pub search_service: Option<Arc<flora_search::SearchService>>,
    /// Real-time messaging service backed by Valkey Pub/Sub (optional).
    pub messaging_service: Option<Arc<flora_messaging::MessagingService>>,
    /// Valkey/Redis connection manager (shared, optional).
    pub valkey: Option<redis::aio::ConnectionManager>,
    /// Transient store for OIDC auth transactions (PKCE state/verifier).
    pub oidc_transactions: OidcTransactionStore,
    /// In-process realtime hub for single-node fanout (best-effort).
    pub realtime_hub: tokio::sync::broadcast::Sender<realtime::HubEnvelope>,
}

impl AppState {
    /// Constrói o emissor de recibos HITL a partir da configuração carregada.
    #[must_use]
    pub fn hitl_receipt_adapter(&self) -> crate::adapters::HitlReceiptAdapter {
        crate::adapters::HitlReceiptAdapter::new(&self.config.hitl.hmac_secret)
    }
}

impl AppState {
    /// Creates a new `AppState` using an existing database pool.
    ///
    /// # Errors
    ///
    /// Returns an error if required services cannot be initialized.
    #[expect(
        clippy::too_many_lines,
        reason = "centralized app wiring; refactor can be done separately"
    )]
    pub async fn new(db_pool: PgPool, config: Config) -> anyhow::Result<Self> {
        // Initialize repositories as Arc<dyn Trait + Send + Sync>
        let _user_repo: Arc<dyn flora_core::traits::UserRepository + Send + Sync> =
            Arc::new(PgUserRepository::new(db_pool.clone()));
        let org_repo: Arc<dyn flora_core::traits::OrganizationRepository + Send + Sync> =
            Arc::new(PgOrganizationRepository::new(db_pool.clone()));
        let membership_repo: Arc<dyn flora_core::traits::MembershipRepository + Send + Sync> =
            Arc::new(PgMembershipRepository::new(db_pool.clone()));
        let role_repo: Arc<dyn flora_core::traits::RoleRepository + Send + Sync> =
            Arc::new(PgRoleRepository::new(db_pool.clone()));
        let session_repo: Arc<dyn flora_core::traits::SessionRepository + Send + Sync> =
            Arc::new(PgSessionRepository::new(db_pool.clone()));
        let workspace_repo: Arc<dyn flora_core::traits::WorkspaceRepository + Send + Sync> =
            Arc::new(PgWorkspaceRepository::new(db_pool.clone()));
        let channel_repo: Arc<dyn flora_core::traits::ChannelRepository + Send + Sync> =
            Arc::new(PgChannelRepository::new(db_pool.clone()));
        let message_repo: Arc<dyn flora_core::traits::MessageRepository + Send + Sync> =
            Arc::new(PgMessageRepository::new(db_pool.clone()));
        let task_repo: Arc<dyn flora_core::traits::TaskRepository + Send + Sync> =
            Arc::new(PgTaskRepository::new(db_pool.clone()));
        let file_repo: Arc<dyn flora_core::traits::FileRepository + Send + Sync> =
            Arc::new(PgFileRepository::new(db_pool.clone()));
        let notification_repo: Arc<dyn flora_core::traits::NotificationRepository + Send + Sync> =
            Arc::new(PgNotificationRepository::new(db_pool.clone()));
        let applied = flora_core::config::bootstrap::apply_startup_password(
            &db_pool,
            &config.bootstrap,
            flora_core::services::PasswordService::shared(),
        )
        .await?;
        tracing::info!(applied = applied, "Startup administrator password recovery");

        // Initialize services
        let rbac_service = Arc::new(RbacService::new(
            Arc::clone(&role_repo),
            Arc::clone(&membership_repo),
        ));

        let session_service = Arc::new(SessionService::new(Arc::clone(&session_repo)));

        let organization_service = Arc::new(OrganizationService::new(
            Arc::clone(&org_repo),
            Arc::clone(&membership_repo),
            Arc::clone(&role_repo),
        ));

        let workspace_service = Arc::new(WorkspaceService::new(Arc::clone(&workspace_repo)));

        let channel_service = Arc::new(ChannelService::new(Arc::clone(&channel_repo)));

        let message_service = Arc::new(MessageService::new(Arc::clone(&message_repo)));

        let task_service = Arc::new(TaskService::new(Arc::clone(&task_repo)));

        let file_service = Arc::new(FileService::new(
            Arc::clone(&file_repo),
            Arc::new(config.storage.clone()),
        ));

        let notification_service =
            Arc::new(NotificationService::new(Arc::clone(&notification_repo)));

        let search_service = if config.search.enabled {
            match flora_search::SearchService::new(
                &config.search.url,
                config.search.api_key.as_deref(),
                &config.search.index_template,
            ) {
                Ok(service) => Some(Arc::new(service)),
                Err(e) => {
                    tracing::warn!(
                        error = %e,
                        "Search service unavailable; continuing in degraded mode"
                    );
                    None
                }
            }
        } else {
            tracing::info!("Search service disabled by configuration");
            None
        };

        let (valkey, messaging_service) = if config.messaging.enabled {
            match redis::Client::open(config.messaging.valkey_url.as_str()) {
                Ok(client) => match client.get_connection_manager().await {
                    Ok(manager) => {
                        let messaging =
                            Arc::new(flora_messaging::MessagingService::new(manager.clone()));
                        (Some(manager), Some(messaging))
                    }
                    Err(e) => {
                        tracing::warn!("Failed to create Valkey connection manager: {e}");
                        (None, None)
                    }
                },
                Err(e) => {
                    tracing::warn!("Failed to create Valkey client: {e}");
                    (None, None)
                }
            }
        } else {
            tracing::info!("Messaging (Valkey) disabled by configuration");
            (None, None)
        };

        let oidc_transactions = OidcTransactionStore::new(valkey.clone());
        let realtime_hub = realtime::create_hub();

        tracing::info!(
            db = true,
            search_enabled = search_service.is_some(),
            messaging_enabled = messaging_service.is_some(),
            "App capabilities initialized"
        );

        Ok(Self {
            db_pool: Arc::new(db_pool),
            config: Arc::new(config),
            rbac_service,
            session_service,
            organization_service,
            workspace_service,
            channel_service,
            message_service,
            task_service,
            file_service,
            notification_service,
            search_service,
            messaging_service,
            valkey,
            oidc_transactions,
            realtime_hub,
        })
    }
}

impl<S> FromRequestParts<S> for AppState
where
    S: Send + Sync + Clone + 'static,
{
    type Rejection = (axum::http::StatusCode, String);

    #[expect(
        clippy::unused_async_trait_impl,
        reason = "axum::extract::FromRequestParts requires async fn"
    )]
    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // The AppState is typically injected via `.with_state()` in the router,
        // so this extractor is rarely needed directly.
        Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "AppState extractor not available".to_string(),
        ))
    }
}
