<script lang="ts">
	import { isValidFileSize } from '$lib/utils/files';
	import { logger } from '$lib/utils/logger';
	import { ApiClient } from '$lib/api/client';
	import type { FileAttachment } from '$lib/types/models';
	import { t } from '$lib/i18n';

	let { onUploadComplete, workspaceId } = $props<{
		onUploadComplete?: (file: FileAttachment, targetWorkspaceId: string) => void;
		workspaceId?: string;
	}>();

	let errorMessage = $state('');

	async function handleUpload(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files?.[0]) {
			errorMessage = '';
			const file = input.files[0];
			if (!isValidFileSize(file.size)) {
				errorMessage = t['file.tooLarge'];
				return;
			}
			const targetWorkspaceId = workspaceId;
			if (!targetWorkspaceId) {
				errorMessage = t['file.workspaceRequired'];
				return;
			}
			// Implement RustFS upload
			try {
				const formData = new FormData();
				formData.append('file', file);
				formData.append('workspace_id', targetWorkspaceId);

				const fileAttachment = await ApiClient.request<FileAttachment>(
					`/api/v1/files/upload?workspace_id=${encodeURIComponent(targetWorkspaceId)}`,
					{
						method: 'POST',
						body: formData
						// Fetch automatically sets multipart/form-data boundary when body is FormData
					}
				);

				if (onUploadComplete) {
					onUploadComplete(fileAttachment, targetWorkspaceId);
				}

				// Reset input
				input.value = '';
			} catch (error) {
				logger.error('FileUpload', 'Upload failed', undefined, error as Error);
				errorMessage = t['file.uploadFailed'];
			}
		}
	}
</script>

{#if errorMessage}
	<div class="error-message mb-2 text-sm text-red-700 dark:text-red-400" role="alert">
		{errorMessage}
	</div>
{/if}
<input type="file" aria-label={t['file.selectInput']} class="text-sm" onchange={handleUpload} />
