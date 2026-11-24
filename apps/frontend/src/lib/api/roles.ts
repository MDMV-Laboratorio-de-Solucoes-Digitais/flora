import { ApiClient } from './client';
import { toRoleItem, type ApiRole } from '$lib/utils/roles';
import type { RoleItem } from '$lib/types/models';

export interface RoleInput {
	name: string;
	description?: string;
	permissions: string[];
}

export async function fetchRoles(): Promise<RoleItem[]> {
	const response = await ApiClient.request<ApiRole[]>('/api/v1/roles');
	return response.map((role) => toRoleItem(role));
}

export async function createRole(input: RoleInput): Promise<RoleItem> {
	const response = await ApiClient.request<ApiRole>('/api/v1/roles', {
		method: 'POST',
		body: JSON.stringify({
			name: input.name.trim(),
			description: input.description?.trim() ? input.description.trim() : undefined,
			permissions: input.permissions
		})
	});
	return toRoleItem(response);
}

export async function updateRole(roleId: string, input: RoleInput): Promise<RoleItem> {
	const response = await ApiClient.request<ApiRole>(`/api/v1/roles/${roleId}`, {
		method: 'PATCH',
		body: JSON.stringify({
			name: input.name.trim(),
			description: input.description?.trim() ?? '',
			permissions: input.permissions
		})
	});
	return toRoleItem(response);
}

export async function deleteRole(roleId: string): Promise<void> {
	await ApiClient.request(`/api/v1/roles/${roleId}`, { method: 'DELETE' });
}
