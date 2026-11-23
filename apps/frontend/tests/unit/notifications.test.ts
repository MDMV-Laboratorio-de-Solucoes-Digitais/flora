import { beforeEach, describe, expect, it } from 'vitest';
import { NotificationStore } from '../../src/lib/state/NotificationStore.svelte';
import type { NotificationItem } from '../../src/lib/types/models';
import {
	notificationEventLabel,
	notificationsQueryPath,
	resolveNotificationHref
} from '../../src/lib/utils/notifications';
import { t } from '$lib/i18n';

function makeNotification(id: string, isRead: boolean): NotificationItem {
	return {
		id,
		eventType: 'Mention',
		targetId: `target-${id}`,
		targetUrl: null,
		isRead,
		createdAt: '2026-11-23T06:00:00Z'
	};
}

describe('notifications helpers', () => {
	it('builds query paths with optional unread_only and clamped limit', () => {
		expect(notificationsQueryPath()).toBe('/api/v1/notifications');
		expect(notificationsQueryPath({ unreadOnly: true, limit: 50 })).toBe(
			'/api/v1/notifications?limit=50&unread_only=true'
		);
		expect(notificationsQueryPath({ limit: 0 })).toBe('/api/v1/notifications?limit=1');
		expect(notificationsQueryPath({ limit: 999 })).toBe('/api/v1/notifications?limit=100');
	});

	it('maps notification event types to pt-BR labels with safe fallback', () => {
		expect(notificationEventLabel('Mention')).toBe(t['notifications.type.Mention']);
		expect(notificationEventLabel('Assignment')).toBe(t['notifications.type.Assignment']);
		expect(notificationEventLabel('Reply')).toBe(t['notifications.type.Reply']);
		expect(notificationEventLabel('FileShare')).toBe(t['notifications.type.FileShare']);
		expect(notificationEventLabel('ContentUpdate')).toBe(t['notifications.type.ContentUpdate']);
		expect(notificationEventLabel('TaskCompleted')).toBe(t['notifications.type.TaskCompleted']);
		expect(notificationEventLabel('Invitation')).toBe(t['notifications.type.Invitation']);
		expect(notificationEventLabel('RoleChange')).toBe(t['notifications.type.RoleChange']);
		expect(notificationEventLabel('WorkspaceActivity')).toBe(
			t['notifications.type.WorkspaceActivity']
		);
		expect(notificationEventLabel('CustomEvent')).toBe('CustomEvent');
	});

	it('normalizes targetUrl and eventType to real internal routes and rejects unsafe links', () => {
		expect(
			resolveNotificationHref({
				eventType: 'Mention',
				targetUrl: '/channels/general/messages/eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee'
			})
		).toBe('/channels');
		expect(
			resolveNotificationHref({
				eventType: 'Assignment',
				targetUrl: '/tasks/33333333-4444-5555-6666-777777777777'
			})
		).toBe('/tasks');
		expect(resolveNotificationHref({ eventType: 'FileShare', targetUrl: '/files' })).toBe('/files');
		expect(resolveNotificationHref({ eventType: 'RoleChange', targetUrl: '/settings' })).toBe(
			'/settings'
		);
		expect(
			resolveNotificationHref({ eventType: 'Reply', targetUrl: 'https://evil.example/phish' })
		).toBe('/channels');
		expect(
			resolveNotificationHref({ eventType: 'Assignment', targetUrl: 'javascript:alert(1)' })
		).toBe('/tasks');
		expect(resolveNotificationHref({ eventType: 'TaskCompleted', targetUrl: '//evil.test' })).toBe(
			'/tasks'
		);
		expect(resolveNotificationHref({ eventType: 'Mention', targetUrl: '/dms' })).toBe('/channels');
		expect(resolveNotificationHref({ eventType: 'Invitation', targetUrl: null })).toBe('/settings');
	});
});

describe('NotificationStore', () => {
	let store: NotificationStore;

	beforeEach(() => {
		store = new NotificationStore();
	});

	it('tracks unreadCount reactively and supports markAsRead, markAllAsRead, and clear', () => {
		store.setNotifications([
			makeNotification('n-1', false),
			makeNotification('n-2', true),
			makeNotification('n-3', false)
		]);
		expect(store.unreadCount).toBe(2);

		store.markAsRead('n-1');
		expect(store.unreadCount).toBe(1);
		expect(store.notifications.find((n) => n.id === 'n-1')?.isRead).toBe(true);

		store.markAllAsRead();
		expect(store.unreadCount).toBe(0);
		expect(store.notifications.every((n) => n.isRead)).toBe(true);

		store.clear();
		expect(store.notifications).toEqual([]);
		expect(store.unreadCount).toBe(0);
	});
});
