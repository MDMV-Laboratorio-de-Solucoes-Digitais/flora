import { describe, expect, it } from 'vitest';
import { t } from '$lib/i18n';
import { MemberStore } from '$lib/state/MemberStore.svelte';
import {
	canManageMember,
	toOrganizationMemberItem,
	validateAddMemberForm
} from '$lib/utils/members';

const sampleMember = (userId: string, roleId = 'role-member', roleName = 'Member') => ({
	userId,
	email: `${userId}@acme.test`,
	displayName: `User ${userId}`,
	roleId,
	roleName,
	joinedAt: '2026-11-24T12:00:00Z'
});

describe('member utilities', () => {
	it('maps API payload into OrganizationMemberItem', () => {
		expect(
			toOrganizationMemberItem({
				user_id: 'u-1',
				email: 'alice@acme.test',
				display_name: 'Alice Admin',
				role_id: 'r-1',
				role_name: 'Admin',
				joined_at: '2026-11-24T12:00:00Z'
			})
		).toEqual({
			userId: 'u-1',
			email: 'alice@acme.test',
			displayName: 'Alice Admin',
			roleId: 'r-1',
			roleName: 'Admin',
			joinedAt: '2026-11-24T12:00:00Z'
		});
	});

	it('prevents self-demotion and self-revocation in UI controls', () => {
		const selfMember = sampleMember('u-self');
		const peerMember = sampleMember('u-peer');

		expect(canManageMember(selfMember, 'u-self')).toBe(false);
		expect(canManageMember(peerMember, 'u-self')).toBe(true);
		expect(canManageMember(peerMember, null)).toBe(false);
		expect(canManageMember(peerMember, undefined)).toBe(false);
	});

	it('validates add member email and role selection with pt-BR keys', () => {
		expect(validateAddMemberForm({ email: '  ', roleId: 'r-1' })).toBe('members.emailInvalid');
		expect(validateAddMemberForm({ email: 'not-an-email', roleId: 'r-1' })).toBe(
			'members.emailInvalid'
		);
		expect(validateAddMemberForm({ email: 'bob@acme.test', roleId: '  ' })).toBe(
			'members.roleRequired'
		);
		expect(validateAddMemberForm({ email: 'bob@acme.test', roleId: 'r-1' })).toBeNull();
		expect(t['members.title']).toBeTypeOf('string');
	});
});

describe('MemberStore', () => {
	it('ignores stale loads after a context switch', async () => {
		const store = new MemberStore();
		store.syncContext('org-a');
		const slow = store.load({
			contextKey: 'org-a',
			fetcher: () => new Promise((resolve) => setTimeout(() => resolve([sampleMember('u-a')]), 20))
		});
		store.syncContext('org-b');
		expect(await slow).toBe(false);
		expect(store.members).toEqual([]);
	});

	it('deduplicates concurrent loads unless forced and updates role/removals locally', async () => {
		const store = new MemberStore();
		store.syncContext('org-a');
		let callCount = 0;
		const fetcher = async () => {
			callCount += 1;
			await new Promise((resolve) => setTimeout(resolve, 15));
			return [sampleMember(`u-${callCount}`)];
		};

		const first = store.load({ contextKey: 'org-a', fetcher });
		const second = store.load({ contextKey: 'org-a', fetcher });
		expect(await Promise.all([first, second])).toEqual([true, true]);
		expect(callCount).toBe(1);

		await store.load({ contextKey: 'org-a', fetcher, shouldForce: true });
		expect(callCount).toBe(2);
		expect(store.members.map((m) => m.userId)).toEqual(['u-2']);

		store.upsert(sampleMember('u-3', 'r-mem', 'Member'));
		store.updateRoleLocal('u-3', 'r-view', 'Viewer');
		expect(store.members.find((m) => m.userId === 'u-3')?.roleName).toBe('Viewer');

		store.remove('u-3');
		expect(store.members.map((m) => m.userId)).toEqual(['u-2']);
	});
});
