import type { NotificationItem } from '../types/models';

export interface LoadNotificationsOptions {
	contextKey: string;
	fetcher: () => Promise<NotificationItem[]>;
	shouldForce?: boolean;
}

export class NotificationStore {
	private activeContextKey: string | null = null;
	private loadSequence = 0;
	private pendingContextKey: string | null = null;
	private pendingLoad: Promise<boolean> | null = null;

	notifications = $state<NotificationItem[]>([]);
	isLoading = $state(false);
	unreadCount = $derived(this.notifications.filter((item) => !item.isRead).length);

	private invalidateState(shouldClearItems: boolean) {
		this.loadSequence += 1;
		this.pendingContextKey = null;
		this.pendingLoad = null;
		this.isLoading = false;
		if (shouldClearItems) {
			this.notifications = [];
		}
	}

	syncContext(nextContextKey: string | null): boolean {
		if (nextContextKey === this.activeContextKey) {
			return false;
		}
		this.activeContextKey = nextContextKey;
		this.invalidateState(true);
		return true;
	}

	async load(options: LoadNotificationsOptions): Promise<boolean> {
		const { contextKey, fetcher, shouldForce = false } = options;
		if (!contextKey || contextKey !== this.activeContextKey) {
			return false;
		}
		if (!shouldForce && this.pendingLoad && this.pendingContextKey === contextKey) {
			return this.pendingLoad;
		}

		const sequence = ++this.loadSequence;
		this.isLoading = true;
		this.pendingContextKey = contextKey;

		const request = (async () => {
			try {
				const items = await fetcher();
				if (sequence !== this.loadSequence || contextKey !== this.activeContextKey) {
					return false;
				}
				this.notifications = [...items];
				return true;
			} finally {
				if (sequence === this.loadSequence) {
					this.isLoading = false;
					this.pendingContextKey = null;
					this.pendingLoad = null;
				}
			}
		})();

		this.pendingLoad = request;
		return request;
	}

	setNotifications(items: NotificationItem[]) {
		this.invalidateState(false);
		this.notifications = [...items];
	}

	markAsRead(id: string) {
		this.invalidateState(false);
		this.notifications = this.notifications.map((item) =>
			item.id === id ? { ...item, isRead: true } : item
		);
	}

	markAllAsRead() {
		this.invalidateState(false);
		this.notifications = this.notifications.map((item) =>
			item.isRead ? item : { ...item, isRead: true }
		);
	}

	clear() {
		this.activeContextKey = null;
		this.invalidateState(true);
	}
}

export const notificationStore = new NotificationStore();
