import { t, type TranslationKey } from '$lib/i18n';
import type { RoleItem } from '$lib/types/models';

/** Canonical permission strings accepted by `POST /api/v1/roles` (mirrors `flora-core` `Permission`). */
export const ROLE_PERMISSION_OPTIONS: readonly string[] = [
	'org:read',
	'org:write',
	'org:admin',
	'workspace:read',
	'workspace:write',
	'workspace:admin',
	'channel:read',
	'channel:write',
	'channel:delete',
	'message:read',
	'message:write',
	'message:edit',
	'message:delete',
	'task:read',
	'task:write',
	'task:assign',
	'task:delete',
	'file:read',
	'file:write',
	'file:delete',
	'search:global',
	'notification:read',
	'role:read',
	'role:write',
	'member:read',
	'member:invite',
	'member:remove'
];

export interface ApiRole {
	id: string;
	name: string;
	description?: string | null;
	permissions: unknown;
	is_builtin: boolean;
}

export function toRoleItem(api: ApiRole): RoleItem {
	const permissions = Array.isArray(api.permissions)
		? api.permissions.filter((value): value is string => typeof value === 'string')
		: [];
	const trimmedDescription = typeof api.description === 'string' ? api.description.trim() : '';
	return {
		id: api.id,
		name: api.name,
		description: trimmedDescription.length > 0 ? trimmedDescription : null,
		permissions,
		isBuiltin: api.is_builtin
	};
}

export function canModifyRole(role: Pick<RoleItem, 'isBuiltin'>): boolean {
	return !role.isBuiltin;
}

export function canDeleteRole(role: Pick<RoleItem, 'isBuiltin'>): boolean {
	return canModifyRole(role);
}

export function pickEditablePermissions(permissions: string[]): string[] {
	return permissions.filter((value) => ROLE_PERMISSION_OPTIONS.includes(value));
}

export function formatRolePermissions(permissions: string[]): string {
	if (permissions.includes('*')) return t['roles.allPermissions'];
	if (permissions.length === 0) return t['roles.noPermissions'];
	return permissions.toSorted((a, b) => a.localeCompare(b)).join(', ');
}

export function validateRoleForm(form: {
	name: string;
	permissions: string[];
}): TranslationKey | null {
	if (form.name.trim().length === 0) return 'roles.nameRequired';
	if (form.permissions.length === 0) return 'roles.permissionsRequired';
	if (form.permissions.some((value) => !ROLE_PERMISSION_OPTIONS.includes(value))) {
		return 'roles.permissionInvalid';
	}
	return null;
}
