import { describe, expect, it } from 'vitest';
import {
	ROLE_PERMISSION_OPTIONS,
	canDeleteRole,
	canModifyRole,
	formatRolePermissions,
	pickEditablePermissions,
	toRoleItem,
	validateRoleForm
} from '../../src/lib/utils/roles';
import { RoleStore } from '../../src/lib/state/RoleStore.svelte';
import { t } from '$lib/i18n';

describe('roles helpers', () => {
	it('maps the API role DTO and normalizes blank descriptions or malformed permissions', () => {
		const role = toRoleItem({
			id: 'r1',
			name: 'Owner',
			description: ' '.repeat(3),
			permissions: ['*'],
			is_builtin: true
		});
		expect(role).toEqual({
			id: 'r1',
			name: 'Owner',
			description: null,
			permissions: ['*'],
			isBuiltin: true
		});
		expect(toRoleItem({ id: 'r2', name: 'X', permissions: { bad: 1 }, is_builtin: false })).toEqual(
			{ id: 'r2', name: 'X', description: null, permissions: [], isBuiltin: false }
		);
	});

	it('only custom roles can be modified or deleted from the UI', () => {
		expect(canModifyRole({ isBuiltin: true })).toBe(false);
		expect(canModifyRole({ isBuiltin: false })).toBe(true);
		expect(canDeleteRole({ isBuiltin: true })).toBe(false);
		expect(canDeleteRole({ isBuiltin: false })).toBe(true);
	});

	it('picks only canonical editable permissions when initializing edit state', () => {
		expect(pickEditablePermissions(['*', 'org:read', 'LegacyPerm', 'task:write'])).toEqual([
			'org:read',
			'task:write'
		]);
	});

	it('formats permissions with wildcard label and stable ordering', () => {
		expect(formatRolePermissions(['*'])).toBe(t['roles.allPermissions']);
		expect(formatRolePermissions([])).toBe(t['roles.noPermissions']);
		expect(formatRolePermissions(['task:write', 'org:read'])).toBe('org:read, task:write');
	});

	it('validates the create and edit role form', () => {
		expect(validateRoleForm({ name: '  ', permissions: ['org:read'] })).toBe('roles.nameRequired');
		expect(validateRoleForm({ name: 'Auditor', permissions: [] })).toBe(
			'roles.permissionsRequired'
		);
		expect(validateRoleForm({ name: 'Auditor', permissions: ['bogus:perm'] })).toBe(
			'roles.permissionInvalid'
		);
		expect(
			validateRoleForm({ name: 'Auditor', permissions: ['org:read', 'task:write'] })
		).toBeNull();
	});

	it('exposes only canonical backend permissions as options (no wildcard)', () => {
		expect(ROLE_PERMISSION_OPTIONS).toContain('role:write');
		expect(ROLE_PERMISSION_OPTIONS).not.toContain('*');
		expect(new Set(ROLE_PERMISSION_OPTIONS).size).toBe(ROLE_PERMISSION_OPTIONS.length);
		expect(ROLE_PERMISSION_OPTIONS).toHaveLength(27);
	});
});

const role = (id: string, isBuiltin = false, name = id) => ({
	id,
	name,
	description: null,
	permissions: ['org:read'],
	isBuiltin
});

describe('RoleStore', () => {
	it('ignores stale loads after a context switch', async () => {
		const store = new RoleStore();
		store.syncContext('org-a');
		const slow = store.load({
			contextKey: 'org-a',
			fetcher: () => new Promise((resolve) => setTimeout(() => resolve([role('a')]), 20))
		});
		store.syncContext('org-b');
		expect(await slow).toBe(false);
		expect(store.roles).toEqual([]);
	});

	it('deduplicates concurrent loads for the same context unless forced', async () => {
		const store = new RoleStore();
		store.syncContext('org-a');
		let callCount = 0;
		const fetcher = async () => {
			callCount += 1;
			await new Promise((resolve) => setTimeout(resolve, 15));
			return [role(`r-${callCount}`)];
		};

		const first = store.load({ contextKey: 'org-a', fetcher });
		const second = store.load({ contextKey: 'org-a', fetcher });
		expect(await Promise.all([first, second])).toEqual([true, true]);
		expect(callCount).toBe(1);

		await store.load({ contextKey: 'org-a', fetcher, shouldForce: true });
		expect(callCount).toBe(2);
		expect(store.roles.map((r) => r.id)).toEqual(['r-2']);
	});

	it('upserts in place and removes roles locally', () => {
		const store = new RoleStore();
		store.syncContext('org-a');
		store.setRoles([role('a', true, 'Owner')]);
		store.upsert(role('b', false, 'Auditor'));
		store.upsert(role('b', false, 'Auditor v2'));
		expect(store.roles.map((r) => `${r.id}:${r.name}`)).toEqual(['a:Owner', 'b:Auditor v2']);
		store.remove('b');
		expect(store.roles.map((r) => r.id)).toEqual(['a']);
	});
});
