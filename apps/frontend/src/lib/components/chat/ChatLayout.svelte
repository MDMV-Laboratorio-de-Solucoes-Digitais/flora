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
	import { onMount, tick } from 'svelte';
	import { Send, Sparkles } from 'lucide-svelte';

	let { channelId = 'default-channel' } = $props<{ channelId?: string }>();

	let newMessage = $state('');
	let isSending = $state(false);
	let attachments = $state<FileAttachment[]>([]);
	let chatContainer = $state<HTMLElement>();

	onMount(async () => {
		try {
			const messages = await ApiClient.request<Message[]>(`/api/v1/channels/${channelId}/messages`);
			for (const message of messages) {
				messageStore.addMessage(message);
			}
			scrollToBottom();
		} catch (error) {
			logger.error('chat', 'Failed to fetch initial messages', undefined, error as Error);
		}
	});

	async function scrollToBottom() {
		await tick();
		if (chatContainer) {
			chatContainer.scrollTop = chatContainer.scrollHeight;
		}
	}

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
			scrollToBottom();
		} catch (error) {
			logger.error('chat', 'Failed to send message', undefined, error as Error);
		} finally {
			isSending = false;
		}
	}
</script>

<div class="relative flex h-full flex-col overflow-hidden bg-background">
	<div class="flex-1 overflow-y-auto" bind:this={chatContainer}>
		<div class="py-4 pb-8">
			{#each messageStore.messages.filter((m) => !m.threadId) as message (message.id)}
				<div class="group flex flex-col px-4 py-2 transition-colors hover:bg-muted/40 md:px-6">
					<div class="flex gap-4">
						<!-- Avatar -->
						<div
							class="flex h-10 w-10 shrink-0 items-center justify-center rounded-md border bg-secondary font-bold text-secondary-foreground"
						>
							{message.authorId.substring(0, 1).toUpperCase()}
						</div>

						<!-- Message Content -->
						<div class="min-w-0 flex-1">
							<div class="mb-1 flex items-baseline space-x-2">
								<span class="text-[15px] font-semibold text-foreground"
									>User {message.authorId}</span
								>
								<span class="text-xs text-muted-foreground"
									>{new Date(message.createdAt).toLocaleTimeString([], {
										hour: '2-digit',
										minute: '2-digit'
									})}</span
								>

								<!-- AI Tag Example -->
								{#if message.content.includes('[AI]')}
									<span
										class="inline-flex items-center gap-1 rounded bg-purple-100 px-1.5 py-0.5 text-[10px] font-medium text-purple-800 dark:bg-purple-900/30 dark:text-purple-300"
									>
										<Sparkles class="h-3 w-3" /> AI Generated
									</span>
								{/if}
							</div>

							<div class="font-sans text-sm leading-relaxed whitespace-pre-wrap text-foreground/90">
								{message.content.replace('[AI]', '')}
							</div>

							{#if message.fileAttachments && message.fileAttachments.length > 0}
								<div class="mt-3 flex flex-wrap gap-2">
									{#each message.fileAttachments as file (file.id)}
										<FilePreview {file} />
									{/each}
								</div>
							{/if}

							<!-- Thread Replies -->
							{#if messageStore.messages.filter((m) => m.threadId === message.id).length > 0}
								<div class="mt-3 space-y-3 border-l-2 border-primary/20 pl-2">
									{#each messageStore.messages.filter((m) => m.threadId === message.id) as reply (reply.id)}
										<div class="flex gap-3">
											<div
												class="flex h-6 w-6 shrink-0 items-center justify-center rounded bg-secondary text-[10px] font-bold"
											>
												{reply.authorId.substring(0, 1).toUpperCase()}
											</div>
											<div class="flex-1">
												<div class="flex items-baseline space-x-2">
													<span class="text-sm font-semibold">User {reply.authorId}</span>
													<span class="text-[10px] text-muted-foreground"
														>{new Date(reply.createdAt).toLocaleTimeString([], {
															hour: '2-digit',
															minute: '2-digit'
														})}</span
													>
												</div>
												<div class="mt-0.5 text-sm text-foreground/80">
													{reply.content}
												</div>
											</div>
										</div>
									{/each}
								</div>
							{/if}

							<div class="mt-2 opacity-0 transition-opacity group-hover:opacity-100">
								<ThreadReply parentMessageId={message.id} {channelId} />
							</div>
						</div>
					</div>
				</div>
			{/each}

			{#if messageStore.messages.length === 0}
				<div
					class="mt-20 flex h-full flex-col items-center justify-center space-y-4 text-muted-foreground"
				>
					<div class="flex h-12 w-12 items-center justify-center rounded-full bg-muted">
						<Send class="h-6 w-6 opacity-50" />
					</div>
					<p>This is the beginning of the channel.</p>
				</div>
			{/if}
		</div>
	</div>

	<!-- Input Area -->
	<div class="bg-background px-4 pt-2 pb-6 md:px-6">
		{#if attachments.length > 0}
			<div class="mb-2 flex flex-wrap gap-2 rounded-md border border-dashed bg-muted/20 p-2">
				{#each attachments as file (file.id)}
					<FilePreview {file} />
				{/each}
			</div>
		{/if}

		<div
			class="relative flex flex-col rounded-lg border bg-background shadow-sm transition-all focus-within:ring-1 focus-within:ring-primary/50"
		>
			<textarea
				bind:value={newMessage}
				placeholder="Message #general..."
				class="min-h-[60px] w-full resize-none scrollbar-thin bg-transparent px-4 py-3 text-sm placeholder:text-muted-foreground focus:outline-none"
				onkeydown={(e) => {
					if (e.key === 'Enter' && !e.shiftKey) {
						e.preventDefault();
						sendMessage();
					}
				}}
				disabled={isSending}></textarea>

			<div class="flex items-center justify-between p-2 pt-0">
				<div class="flex items-center gap-1">
					<FileUpload onUploadComplete={(file) => (attachments = [...attachments, file])} />
				</div>
				<Button
					size="sm"
					onclick={sendMessage}
					disabled={isSending || (!newMessage.trim() && attachments.length === 0)}
					class="h-8 gap-1.5 px-3"
				>
					<Send class="h-3.5 w-3.5" />
					Send
				</Button>
			</div>
		</div>
		<div
			class="mt-2 flex items-center justify-center gap-1 text-center text-[10px] font-medium text-muted-foreground"
		>
			<span class="inline-block h-1.5 w-1.5 rounded-full bg-green-500"></span>
			WebSockets connected to Flora locally
		</div>
	</div>
</div>
