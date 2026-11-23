import type { NotificationItem } from '../types/models';

export class NotificationStore {
	notifications = $state<NotificationItem[]>([]);
	unreadCount = $derived(this.notifications.filter((item) => !item.isRead).length);

	setNotifications(items: NotificationItem[]) {
		this.notifications = [...items];
	}

	markAsRead(id: string) {
		this.notifications = this.notifications.map((item) =>
			item.id === id ? { ...item, isRead: true } : item
		);
	}

	markAllAsRead() {
		this.notifications = this.notifications.map((item) =>
			item.isRead ? item : { ...item, isRead: true }
		);
	}

	clear() {
		this.notifications = [];
	}
}

export const notificationStore = new NotificationStore();
