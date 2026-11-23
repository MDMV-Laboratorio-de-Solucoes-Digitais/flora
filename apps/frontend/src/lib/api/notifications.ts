import { ApiClient } from './client';
import { toNotification, type ApiNotification } from './mappers';
import { notificationsQueryPath } from '$lib/utils/notifications';
import type { NotificationItem } from '$lib/types/models';

export async function fetchOrganizationNotifications(limit = 50): Promise<NotificationItem[]> {
	const response = await ApiClient.request<ApiNotification[]>(notificationsQueryPath({ limit }));
	return response.map((item) => toNotification(item));
}

export async function markNotificationAsRead(notificationId: string): Promise<void> {
	await ApiClient.request(`/api/v1/notifications/${notificationId}/read`, {
		method: 'POST'
	});
}

export async function markAllOrganizationNotificationsAsRead(): Promise<void> {
	await ApiClient.request('/api/v1/notifications/read-all', {
		method: 'POST'
	});
}
