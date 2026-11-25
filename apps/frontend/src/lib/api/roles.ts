import { ApiClient } from './client';
import { toOrganizationMemberItem, type ApiOrganizationMember } from '$lib/utils/members';
import { toRoleItem, type ApiRole } from '$lib/utils/roles';
import type { OrganizationMemberItem, RoleItem } from '$lib/types/models';

export interface RoleInput {
	name: string;
	description?: string;
	permissions: string[];
}

export interface AddOrganizationMemberInput {
	email: string;
	roleId: string;
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
	const response = await ApiClient.request<ApiRole>(`/api/v1/roles/${encodeURIComponent(roleId)}`, {
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
	await ApiClient.request(`/api/v1/roles/${encodeURIComponent(roleId)}`, { method: 'DELETE' });
}

export async function fetchOrganizationMembers(): Promise<OrganizationMemberItem[]> {
	const response = await ApiClient.request<ApiOrganizationMember[]>('/api/v1/roles/members');
	return response.map((member) => toOrganizationMemberItem(member));
}

export async function addOrganizationMember(
	input: AddOrganizationMemberInput
): Promise<OrganizationMemberItem> {
	const response = await ApiClient.request<ApiOrganizationMember>('/api/v1/roles/members', {
		method: 'POST',
		body: JSON.stringify({
			email: input.email.trim().toLowerCase(),
			role_id: input.roleId
		})
	});
	return toOrganizationMemberItem(response);
}

export async function assignRoleToMember(roleId: string, userId: string): Promise<void> {
	await ApiClient.request(`/api/v1/roles/${encodeURIComponent(roleId)}/assign`, {
		method: 'POST',
		body: JSON.stringify({ user_id: userId })
	});
}

export async function revokeRoleFromMember(roleId: string, userId: string): Promise<void> {
	await ApiClient.request(`/api/v1/roles/${encodeURIComponent(roleId)}/revoke`, {
		method: 'POST',
		body: JSON.stringify({ user_id: userId })
	});
}
