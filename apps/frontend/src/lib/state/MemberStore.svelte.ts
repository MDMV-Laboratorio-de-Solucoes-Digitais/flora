import type { OrganizationMemberItem } from '../types/models';

export interface LoadMembersOptions {
	contextKey: string;
	fetcher: () => Promise<OrganizationMemberItem[]>;
	shouldForce?: boolean;
}

export class MemberStore {
	private activeContextKey: string | null = null;
	private loadSequence = 0;
	private pendingContextKey: string | null = null;
	private pendingLoad: Promise<boolean> | null = null;

	members = $state<OrganizationMemberItem[]>([]);
	isLoading = $state(false);

	private invalidateState(shouldClearItems: boolean) {
		this.loadSequence += 1;
		this.pendingContextKey = null;
		this.pendingLoad = null;
		this.isLoading = false;
		if (shouldClearItems) {
			this.members = [];
		}
	}

	syncContext(nextContextKey: string | null): boolean {
		if (nextContextKey === this.activeContextKey) return false;
		this.activeContextKey = nextContextKey;
		this.invalidateState(true);
		return true;
	}

	async load(options: LoadMembersOptions): Promise<boolean> {
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
				this.members = [...items];
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

	setMembers(items: OrganizationMemberItem[]) {
		this.invalidateState(false);
		this.members = [...items];
	}

	upsert(member: OrganizationMemberItem) {
		this.invalidateState(false);
		const hasMember = this.members.some((item) => item.userId === member.userId);
		this.members = hasMember
			? this.members.map((item) => (item.userId === member.userId ? member : item))
			: [...this.members, member];
	}

	updateRoleLocal(userId: string, roleId: string, roleName: string) {
		this.invalidateState(false);
		this.members = this.members.map((item) =>
			item.userId === userId ? { ...item, roleId, roleName } : item
		);
	}

	remove(userId: string) {
		this.invalidateState(false);
		this.members = this.members.filter((item) => item.userId !== userId);
	}

	clear() {
		this.activeContextKey = null;
		this.invalidateState(true);
	}
}

export const memberStore = new MemberStore();
