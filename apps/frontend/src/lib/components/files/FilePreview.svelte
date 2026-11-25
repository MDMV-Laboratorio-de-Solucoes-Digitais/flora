<script lang="ts">
	import { ApiClient } from '$lib/api/client';
	import { apiErrorMessage, t } from '$lib/i18n';
	import type { FileAttachment } from '$lib/types/models';

	let { file, onDelete } = $props<{
		file: FileAttachment;
		onDelete?: (deleted: FileAttachment) => void;
	}>();

	let errorMessage = $state<string | null>(null);
	let isConfirmingDelete = $state(false);
	let isDeleting = $state(false);

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

	async function remove() {
		errorMessage = null;
		if (!isConfirmingDelete) {
			isConfirmingDelete = true;
			return;
		}
		isDeleting = true;
		try {
			await ApiClient.request(`/api/v1/files/${file.id}`, { method: 'DELETE' });
			onDelete?.(file);
			isConfirmingDelete = false;
		} catch (error) {
			errorMessage = apiErrorMessage(error, 'file.deleteFailed');
		} finally {
			isDeleting = false;
		}
	}

	function cancelDelete() {
		isConfirmingDelete = false;
		errorMessage = null;
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
	{#if isConfirmingDelete}
		<button
			type="button"
			onclick={remove}
			disabled={isDeleting}
			aria-label={`${t['file.deleteConfirm']} ${file.fileName}`}
			class="text-red-700 underline hover:text-red-800 disabled:opacity-50 dark:text-red-400 dark:hover:text-red-300"
		>
			{t['file.deleteConfirm']}
		</button>
		<button
			type="button"
			onclick={cancelDelete}
			disabled={isDeleting}
			aria-label={`${t['file.deleteCancel']} ${file.fileName}`}
			class="text-muted-foreground underline hover:text-foreground disabled:opacity-50"
		>
			{t['file.deleteCancel']}
		</button>
	{:else}
		<button
			type="button"
			onclick={remove}
			aria-label={`${t['file.delete']} ${file.fileName}`}
			class="text-red-700 underline hover:text-red-800 dark:text-red-400 dark:hover:text-red-300"
		>
			{t['file.delete']}
		</button>
	{/if}
</div>
{#if errorMessage}
	<p role="alert" class="mt-1 text-red-700 dark:text-red-400">{errorMessage}</p>
{/if}
