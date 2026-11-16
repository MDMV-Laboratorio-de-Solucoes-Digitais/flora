<script lang="ts">
	import { createApiClient } from '$lib/api/client';
	import { createAuthState } from '$lib/state';
	import { logger } from '$lib/logging';
	import { isValidFileSize } from '$lib/utils/files';
	import type { FileAttachment } from '$lib/types/models';

	let { onUploadComplete, workspaceId } = $props<{
		onUploadComplete?: (file: FileAttachment) => void;
		workspaceId?: string;
	}>();
	let uploading = $state(false);
	let error = $state<string | null>(null);

	async function handleFileUpload(event: Event) {
		const authState = createAuthState();
		if (!authState.token) return;
		const apiClient = createApiClient(authState.token);

		const input = event.target as HTMLInputElement;
		const file = input.files?.[0];
		if (!file) return;

		error = null;
		if (!workspaceId) {
			error = 'Workspace is required before uploading files.';
			return;
		}

		try {
			uploading = true;

			if (!isValidFileSize(file.size)) {
				error = 'File size exceeds 50MB limit';
				return;
			}

			const formData = new FormData();
			formData.append('file', file);
			// Include in both query and body for compatibility/robustness
			formData.append('workspace_id', workspaceId);

			try {
				const response = await apiClient.request<FileAttachment>(
					`/api/v1/files/upload?workspace_id=${encodeURIComponent(workspaceId)}`,
					{
						method: 'POST',
						body: formData
					}
				);

				if (onUploadComplete) {
					onUploadComplete(response);
				}

				input.value = '';
			} catch (e) {
				logger.error('FileUpload', 'Upload failed', undefined, e as Error);
				error = 'Upload failed. Please try again.';
			}
		} catch (e) {
			logger.error('FileUpload', 'Unexpected upload error', undefined, e as Error);
			error = 'Unexpected error. Please try again.';
		} finally {
			uploading = false;
		}
	}
</script>

<div>
	<label>
		<span>Upload File</span>
		<input
			type="file"
			disabled={uploading}
			onchange={handleFileUpload}
			aria-label="Upload file"
		/>
	</label>
	{#if uploading}
		<p>Uploading...</p>
	{/if}
	{#if error}
		<p class="error">{error}</p>
	{/if}
</div>

<style>
	.error {
		color: red;
	}
</style>
