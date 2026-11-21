<script lang="ts">
	import { ApiClient } from '$lib/api/client';
	import type { ApiMessage } from '$lib/api/mappers';
	import { logger } from '$lib/utils/logger';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { t } from '$lib/i18n';

	let { parentMessageId, channelId, onSent } = $props<{
		parentMessageId: string;
		channelId: string;
		onSent?: (message: ApiMessage) => void | Promise<void>;
	}>();

	let replyContent = $state('');
	let isSubmitting = $state(false);

	async function sendReply() {
		if (!channelId || !replyContent.trim()) return;
		isSubmitting = true;
		try {
			const created = await ApiClient.request<ApiMessage>(
				`/api/v1/channels/${channelId}/messages`,
				{
					method: 'POST',
					body: JSON.stringify({ content: replyContent.trim(), thread_id: parentMessageId })
				}
			);
			replyContent = '';
			await onSent?.(created);
		} catch (error) {
			logger.error(
				'chat',
				'Failed to send thread reply',
				{ channelId, parentMessageId },
				error as Error
			);
		} finally {
			isSubmitting = false;
		}
	}
</script>

<div class="mt-2 flex gap-2 border-l-2 pl-4">
	<Input
		class="h-8 text-sm"
		bind:value={replyContent}
		placeholder={t['chat.replyPlaceholder']}
		onkeydown={(event_: KeyboardEvent) => event_.key === 'Enter' && sendReply()}
		disabled={isSubmitting || !channelId}
	/>
	<Button size="sm" class="h-8" onclick={sendReply} disabled={isSubmitting || !channelId}
		>{t['chat.reply']}</Button
	>
</div>
