import { t } from '$lib/i18n';

const EVENT_LABELS: Readonly<Record<string, string>> = {
	Mention: t['notifications.type.Mention'],
	Assignment: t['notifications.type.Assignment'],
	Reply: t['notifications.type.Reply'],
	FileShare: t['notifications.type.FileShare'],
	ContentUpdate: t['notifications.type.ContentUpdate'],
	TaskCompleted: t['notifications.type.TaskCompleted'],
	Invitation: t['notifications.type.Invitation'],
	RoleChange: t['notifications.type.RoleChange'],
	WorkspaceActivity: t['notifications.type.WorkspaceActivity']
};

const SAFE_ROUTE_PREFIXES = [
	'/channels',
	'/tasks',
	'/files',
	'/settings',
	'/notifications'
] as const;

export interface NotificationContextInput {
	isAuthenticated: boolean;
	token: string | null;
	organizationId: string | null;
	isSessionLocked: boolean;
	isInvalidated: boolean;
}

export function buildNotificationContextKey(context: NotificationContextInput): string | null {
	if (
		!context.isAuthenticated ||
		!context.token ||
		!context.organizationId ||
		context.isSessionLocked ||
		context.isInvalidated
	) {
		return null;
	}
	return `${context.organizationId}|${context.token}`;
}

export function notificationsQueryPath(options?: { unreadOnly?: boolean; limit?: number }): string {
	const parameters = new URLSearchParams();
	if (options?.limit !== undefined && Number.isFinite(options.limit)) {
		const clamped = Math.min(100, Math.max(1, Math.trunc(options.limit)));
		parameters.set('limit', String(clamped));
	}
	if (options?.unreadOnly) {
		parameters.set('unread_only', 'true');
	}
	const query = parameters.toString();
	return query ? `/api/v1/notifications?${query}` : '/api/v1/notifications';
}

export function notificationEventLabel(eventType: string): string {
	return EVENT_LABELS[eventType] ?? eventType;
}

export function resolveNotificationHref(notification: {
	eventType: string;
	targetUrl?: string | null;
}): string {
	const rawUrl = notification.targetUrl?.trim();
	if (rawUrl && rawUrl.startsWith('/') && !rawUrl.startsWith('//') && !rawUrl.includes('\\')) {
		const lower = rawUrl.toLowerCase();
		for (const prefix of SAFE_ROUTE_PREFIXES) {
			if (lower === prefix || lower.startsWith(`${prefix}/`) || lower.startsWith(`${prefix}?`)) {
				return prefix;
			}
		}
	}

	switch (notification.eventType) {
		case 'Assignment':
		case 'TaskCompleted': {
			return '/tasks';
		}
		case 'FileShare': {
			return '/files';
		}
		case 'Invitation':
		case 'RoleChange': {
			return '/settings';
		}
		default: {
			return '/channels';
		}
	}
}
