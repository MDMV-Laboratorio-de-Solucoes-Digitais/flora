<script lang="ts">
	import { ApiClient } from '$lib/api/client';
	import { apiErrorMessage, t } from '$lib/i18n';
	import type { FileAttachment } from '$lib/types/models';

	let { file } = $props<{ file: FileAttachment }>();
	let errorMessage = $state<string | null>(null);

	async function download() {
		errorMessage = null;
		try {
			const blob = await ApiClient.downloadBlob(`/api/v1/files/download/${file.id}`);
			const url = URL.createObjectURL(blob);
			const anchor = document.createElement('a');
			anchor.href = url;
			anchor.download = file.fileName;
			document.body.append(anchor);
			anchor.click();
			anchor.remove();
			setTimeout(() => URL.revokeObjectURL(url), 1000);
		} catch (error) {
			errorMessage = apiErrorMessage(error, 'file.downloadFailed');
		}
	}
</script>

<div class="flex items-center gap-2 rounded-md border bg-background p-2 text-sm">
	<svg class="h-4 w-4" aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24"
		><path
			stroke-linecap="round"
			stroke-linejoin="round"
			stroke-width="2"
			d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13"
		></path></svg
	>
	<div class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap">
		{file.fileName}
	</div>
	<button
		type="button"
		onclick={download}
		class="text-blue-700 underline hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
	>
		{t['file.download']}
	</button>
</div>
{#if errorMessage}
	<p role="alert" class="mt-1 text-red-700 dark:text-red-400">{errorMessage}</p>
{/if}
