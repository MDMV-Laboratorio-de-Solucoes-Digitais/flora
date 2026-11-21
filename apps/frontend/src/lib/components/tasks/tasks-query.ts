/** Builds the task-list endpoint for the workspace currently in scope. */
export function tasksEndpoint(workspaceId: string | null): string {
	if (!workspaceId) return '/api/v1/tasks';
	return `/api/v1/tasks?workspace_id=${encodeURIComponent(workspaceId)}`;
}
