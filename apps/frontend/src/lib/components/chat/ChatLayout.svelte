<script lang="ts">
	import { messageStore } from '$lib/state/MessageStore.svelte';
	import { ApiClient } from '$lib/api/client';
	import { logger } from '$lib/utils/logger';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import ThreadReply from './thread/ThreadReply.svelte';
	import FileUpload from '../files/FileUpload.svelte';
	import FilePreview from '../files/FilePreview.svelte';
	import type { FileAttachment, Message } from '$lib/types/models';
	import { onMount } from 'svelte';

	let { channelId = 'default-channel' } = $props<{ channelId?: string }>();

	let newMessage = $state('');
	let isSending = $state(false);
	let attachments = $state<FileAttachment[]>([]);

	onMount(async () => {
		try {
			const messages = await ApiClient.request<Message[]>(`/api/v1/channels/${channelId}/messages`);
			for (const message of messages) {
				messageStore.addMessage(message);
			}
		} catch (error) {
			logger.error('chat', 'Failed to fetch initial messages', undefined, error as Error);
		}
	});

	async function sendMessage() {
		if (!newMessage.trim() && attachments.length === 0) return;
		isSending = true;
		try {
			await ApiClient.request(`/api/v1/channels/${channelId}/messages`, {
				method: 'POST',
				body: JSON.stringify({
					content: newMessage,
					fileIds: attachments.map((a) => a.id)
				})
			});
			newMessage = '';
			attachments = [];
		} catch (error) {
			logger.error('chat', 'Failed to send message', undefined, error as Error);
		} finally {
			isSending = false;
		}
	}
</script>

<div class="bg-background flex h-[calc(100vh-4rem)] flex-col overflow-hidden rounded-lg border">
	<div class="flex-1 space-y-4 overflow-y-auto p-4">
		{#each messageStore.messages.filter((m) => !m.threadId) as message (message.id)}
			<div class="flex flex-col space-y-1">
				<div class="flex items-center space-x-2">
					<span class="text-sm font-semibold">User {message.authorId}</span>
					<span class="text-muted-foreground text-xs"
						>{new Date(message.createdAt).toLocaleTimeString()}</span
					>
				</div>
				<div class="bg-muted max-w-[80%] rounded-lg rounded-tl-none p-3">
					{message.content}
					{#if message.fileAttachments && message.fileAttachments.length > 0}
						<div class="mt-2 space-y-2">
							{#each message.fileAttachments as file (file.id)}
								<FilePreview {file} />
							{/each}
						</div>
					{/if}
				</div>

				{#each messageStore.messages.filter((m) => m.threadId === message.id) as reply (reply.id)}
					<div class="mt-2 border-l-2 pl-4">
						<div class="flex items-center space-x-2">
							<span class="text-xs font-semibold">User {reply.authorId}</span>
							<span class="text-muted-foreground text-[10px]"
								>{new Date(reply.createdAt).toLocaleTimeString()}</span
							>
						</div>
						<div class="bg-muted/50 inline-block rounded-lg rounded-tl-none p-2 text-sm">
							{reply.content}
							{#if reply.fileAttachments && reply.fileAttachments.length > 0}
								<div class="mt-2 space-y-2">
									{#each reply.fileAttachments as file (file.id)}
										<FilePreview {file} />
									{/each}
								</div>
							{/if}
						</div>
					</div>
				{/each}

				<ThreadReply parentMessageId={message.id} {channelId} />
			</div>
		{/each}
		{#if messageStore.messages.length === 0}
			<div class="text-muted-foreground flex h-full items-center justify-center">
				No messages yet.
			</div>
		{/if}
	</div>

	<div class="border-t p-4">
		{#if attachments.length > 0}
			<div class="mb-2 flex flex-wrap gap-2">
				{#each attachments as file (file.id)}
					<FilePreview {file} />
				{/each}
			</div>
		{/if}
		<div class="flex items-center gap-2">
			<FileUpload onUploadComplete={(file) => (attachments = [...attachments, file])} />
			<Input
				bind:value={newMessage}
				placeholder="Type a message..."
				onkeydown={(event_: KeyboardEvent) => event_.key === 'Enter' && sendMessage()}
				disabled={isSending}
			/>
			<Button onclick={sendMessage} disabled={isSending}>Send</Button>
		</div>
	</div>
</div>
