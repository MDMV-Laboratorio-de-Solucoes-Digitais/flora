import type { TranslationKey } from '$lib/i18n';
import type { OrganizationMemberItem } from '$lib/types/models';

export interface ApiOrganizationMember {
	user_id: string;
	email: string;
	display_name: string;
	role_id: string;
	role_name: string;
	joined_at: string;
}

export function toOrganizationMemberItem(api: ApiOrganizationMember): OrganizationMemberItem {
	return {
		userId: api.user_id,
		email: api.email,
		displayName: api.display_name,
		roleId: api.role_id,
		roleName: api.role_name,
		joinedAt: api.joined_at
	};
}

export function canManageMember(
	member: Pick<OrganizationMemberItem, 'userId'>,
	actorUserId: string | null | undefined
): boolean {
	return Boolean(actorUserId && member.userId !== actorUserId);
}

export function validateAddMemberForm(form: {
	email: string;
	roleId: string;
}): TranslationKey | null {
	const email = form.email.trim();
	if (email.length === 0 || !email.includes('@')) {
		return 'members.emailInvalid';
	}
	if (form.roleId.trim().length === 0) {
		return 'members.roleRequired';
	}
	return null;
}
