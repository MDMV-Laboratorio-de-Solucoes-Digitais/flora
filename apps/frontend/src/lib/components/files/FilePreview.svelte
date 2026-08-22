<script lang="ts">
	import type { FileAttachment } from '$lib/types/models';

	let { file } = $props<{ file: FileAttachment }>();

	function getDownloadUrl() {
		return `/api/v1/files/download/${file.id}`;
	}
</script>

<div class="bg-background flex items-center gap-2 rounded-md border p-2 text-sm">
	<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"
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
	<a href={getDownloadUrl()} class="text-blue-500 hover:underline" download> Download </a>
</div>
