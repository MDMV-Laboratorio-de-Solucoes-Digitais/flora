import type { RoleItem } from '../types/models';

export interface LoadRolesOptions {
	contextKey: string;
	fetcher: () => Promise<RoleItem[]>;
	shouldForce?: boolean;
}

export class RoleStore {
	private activeContextKey: string | null = null;
	private loadSequence = 0;
	private pendingContextKey: string | null = null;
	private pendingLoad: Promise<boolean> | null = null;

	roles = $state<RoleItem[]>([]);
	isLoading = $state(false);

	private invalidateState(shouldClearItems: boolean) {
		this.loadSequence += 1;
		this.pendingContextKey = null;
		this.pendingLoad = null;
		this.isLoading = false;
		if (shouldClearItems) {
			this.roles = [];
		}
	}

	syncContext(nextContextKey: string | null): boolean {
		if (nextContextKey === this.activeContextKey) return false;
		this.activeContextKey = nextContextKey;
		this.invalidateState(true);
		return true;
	}

	async load(options: LoadRolesOptions): Promise<boolean> {
		const { contextKey, fetcher, shouldForce = false } = options;
		if (!contextKey || contextKey !== this.activeContextKey) return false;
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
				this.roles = [...items];
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

	setRoles(items: RoleItem[]) {
		this.invalidateState(false);
		this.roles = [...items];
	}

	upsert(role: RoleItem) {
		this.invalidateState(false);
		const hasRole = this.roles.some((item) => item.id === role.id);
		this.roles = hasRole
			? this.roles.map((item) => (item.id === role.id ? role : item))
			: [...this.roles, role];
	}

	add(role: RoleItem) {
		this.upsert(role);
	}

	remove(id: string) {
		this.invalidateState(false);
		this.roles = this.roles.filter((item) => item.id !== id);
	}

	clear() {
		this.activeContextKey = null;
		this.invalidateState(true);
	}
}

export const roleStore = new RoleStore();
