type StoredAuth = {
	token: string | null;
	user: Record<string, unknown> | null;
	organizationId: string | null;
	workspaceId: string | null;
};

const STORAGE_KEY = 'flora.auth.v1';

function canUseStorage(): boolean {
	return globalThis.window !== undefined;
}

function loadStoredAuth(): StoredAuth {
	if (!canUseStorage()) {
		return { token: null, user: null, organizationId: null, workspaceId: null };
	}
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (!raw) return { token: null, user: null, organizationId: null, workspaceId: null };
		const parsed = JSON.parse(raw) as Partial<StoredAuth>;
		return {
			token: typeof parsed.token === 'string' ? parsed.token : null,
			user: parsed.user && typeof parsed.user === 'object' ? parsed.user : null,
			organizationId: typeof parsed.organizationId === 'string' ? parsed.organizationId : null,
			workspaceId: typeof parsed.workspaceId === 'string' ? parsed.workspaceId : null
		};
	} catch {
		return { token: null, user: null, organizationId: null, workspaceId: null };
	}
}

export class AuthState {
	token = $state<string | null>(null);
	user = $state<Record<string, unknown> | null>(null);
	organizationId = $state<string | null>(null);
	workspaceId = $state<string | null>(null);
	isAuthenticated = $state(false);
	isSessionLocked = $state(false);
	isInvalidated = $state(false);
	gracePeriodTimeout: ReturnType<typeof setTimeout> | null = null;

	constructor() {
		const stored = loadStoredAuth();
		this.token = stored.token;
		this.user = stored.user;
		this.organizationId = stored.organizationId;
		this.workspaceId = stored.workspaceId;
		this.isAuthenticated = Boolean(stored.token);
	}

	private persist() {
		if (!canUseStorage()) return;
		const payload: StoredAuth = {
			token: this.token,
			user: this.user,
			organizationId: this.organizationId,
			workspaceId: this.workspaceId
		};
		try {
			localStorage.setItem(STORAGE_KEY, JSON.stringify(payload));
		} catch {
			// ignore storage failures in minimal slice
		}
	}

	login(token: string, user: Record<string, unknown>, organizationId: string | null = null) {
		this.token = token;
		this.user = user;
		this.organizationId = organizationId;
		this.isAuthenticated = true;
		this.isSessionLocked = false;
		this.isInvalidated = false;
		if (this.gracePeriodTimeout) {
			clearTimeout(this.gracePeriodTimeout);
			this.gracePeriodTimeout = null;
		}
		this.persist();
	}

	setSession(token: string, user: Record<string, unknown>, organizationId: string | null = null) {
		this.login(token, user, organizationId);
	}

	setOrganization(token: string, organizationId: string) {
		this.token = token;
		this.organizationId = organizationId;
		this.workspaceId = null;
		this.isAuthenticated = true;
		this.isSessionLocked = false;
		if (this.gracePeriodTimeout) {
			clearTimeout(this.gracePeriodTimeout);
			this.gracePeriodTimeout = null;
		}
		this.persist();
	}

	setWorkspace(workspaceId: string | null) {
		this.workspaceId = workspaceId;
		this.persist();
	}

	logout() {
		this.token = null;
		this.user = null;
		this.organizationId = null;
		this.workspaceId = null;
		this.isAuthenticated = false;
		this.isSessionLocked = false;
		if (this.gracePeriodTimeout) {
			clearTimeout(this.gracePeriodTimeout);
			this.gracePeriodTimeout = null;
		}
		if (canUseStorage()) {
			try {
				localStorage.removeItem(STORAGE_KEY);
			} catch {
				// ignore
			}
		}
	}

	lockSession() {
		if (this.isSessionLocked) return;
		this.isSessionLocked = true;

		// 5 minute grace period before full logout
		this.gracePeriodTimeout = setTimeout(
			() => {
				this.logout();
			},
			5 * 60 * 1000
		);
	}

	invalidate() {
		this.isInvalidated = true;
		this.logout();
	}
}

export const authState = new AuthState();
