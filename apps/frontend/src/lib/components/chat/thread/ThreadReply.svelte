<script lang="ts">
	import { ApiClient } from '$lib/api/client';
	import { logger } from '$lib/utils/logger';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';

	let { parentMessageId, channelId } = $props<{ parentMessageId: string; channelId: string }>();

	let replyContent = $state('');
	let isSubmitting = $state(false);

	async function sendReply() {
		if (!replyContent.trim()) return;
		isSubmitting = true;
		try {
			await ApiClient.request(`/api/v1/channels/${channelId}/messages`, {
				method: 'POST',
				body: JSON.stringify({ content: replyContent, threadId: parentMessageId })
			});
			replyContent = '';
		} catch (error) {
			logger.error('chat', 'Failed to send thread reply', undefined, error as Error);
		} finally {
			isSubmitting = false;
		}
	}
</script>

<div class="mt-2 flex gap-2 border-l-2 pl-4">
	<Input
		class="h-8 text-sm"
		bind:value={replyContent}
		placeholder="Reply to thread..."
		onkeydown={(event_: KeyboardEvent) => event_.key === 'Enter' && sendReply()}
		disabled={isSubmitting}
	/>
	<Button size="sm" class="h-8" onclick={sendReply} disabled={isSubmitting}>Reply</Button>
</div>
