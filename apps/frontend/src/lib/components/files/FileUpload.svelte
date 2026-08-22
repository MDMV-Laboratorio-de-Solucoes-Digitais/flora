<script lang="ts">
	import { isValidFileSize } from '$lib/utils/files';
	import { logger } from '$lib/utils/logger';
	import { ApiClient } from '$lib/api/client';
	import type { FileAttachment } from '$lib/types/models';

	let { onUploadComplete } = $props<{ onUploadComplete?: (file: FileAttachment) => void }>();

	let errorMessage = $state('');

	async function handleUpload(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files?.[0]) {
			errorMessage = '';
			const file = input.files[0];
			if (!isValidFileSize(file.size)) {
				errorMessage = 'File too large. Maximum size is 50MB.';
				return;
			}
			// Implement RustFS upload
			try {
				const formData = new FormData();
				formData.append('file', file);

				const fileAttachment = await ApiClient.request<FileAttachment>('/api/v1/files/upload', {
					method: 'POST',
					body: formData
					// Fetch automatically sets multipart/form-data boundary when body is FormData
				});

				if (onUploadComplete) {
					onUploadComplete(fileAttachment);
				}

				// Reset input
				input.value = '';
			} catch (error) {
				logger.error('FileUpload', 'Upload failed', undefined, error as Error);
				errorMessage = 'Upload failed.';
			}
		}
	}
</script>

{#if errorMessage}
	<div class="error-message mb-2 text-sm text-red-500" role="alert">{errorMessage}</div>
{/if}
<input type="file" class="text-sm" onchange={handleUpload} />
