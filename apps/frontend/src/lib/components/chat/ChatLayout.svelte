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

<div class="bg-background flex h-full flex-col overflow-hidden relative">
	<div class="flex-1 overflow-y-auto" bind:this={chatContainer}>
		<div class="py-4 pb-8">
			{#each messageStore.messages.filter((m) => !m.threadId) as message (message.id)}
				<div class="group flex flex-col hover:bg-muted/40 transition-colors py-2 px-4 md:px-6">
					<div class="flex gap-4">
						<!-- Avatar -->
						<div class="h-10 w-10 shrink-0 rounded-md bg-secondary flex items-center justify-center font-bold text-secondary-foreground border">
							{message.authorId.substring(0, 1).toUpperCase()}
						</div>
						
						<!-- Message Content -->
						<div class="flex-1 min-w-0">
							<div class="flex items-baseline space-x-2 mb-1">
								<span class="text-[15px] font-semibold text-foreground">User {message.authorId}</span>
								<span class="text-muted-foreground text-xs">{new Date(message.createdAt).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}</span>
								
								<!-- AI Tag Example -->
								{#if message.content.includes('[AI]')}
									<span class="inline-flex items-center gap-1 rounded bg-purple-100 dark:bg-purple-900/30 px-1.5 py-0.5 text-[10px] font-medium text-purple-800 dark:text-purple-300">
										<Sparkles class="h-3 w-3" /> AI Generated
									</span>
								{/if}
							</div>
							
							<div class="text-sm text-foreground/90 leading-relaxed whitespace-pre-wrap font-sans">
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
								<div class="mt-3 pl-2 border-l-2 border-primary/20 space-y-3">
									{#each messageStore.messages.filter((m) => m.threadId === message.id) as reply (reply.id)}
										<div class="flex gap-3">
											<div class="h-6 w-6 shrink-0 rounded bg-secondary flex items-center justify-center text-[10px] font-bold">
												{reply.authorId.substring(0, 1).toUpperCase()}
											</div>
											<div class="flex-1">
												<div class="flex items-baseline space-x-2">
													<span class="text-sm font-semibold">User {reply.authorId}</span>
													<span class="text-muted-foreground text-[10px]">{new Date(reply.createdAt).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}</span>
												</div>
												<div class="text-sm text-foreground/80 mt-0.5">
													{reply.content}
												</div>
											</div>
										</div>
									{/each}
								</div>
							{/if}

							<div class="mt-2 opacity-0 group-hover:opacity-100 transition-opacity">
								<ThreadReply parentMessageId={message.id} {channelId} />
							</div>
						</div>
					</div>
				</div>
			{/each}
			
			{#if messageStore.messages.length === 0}
				<div class="flex flex-col h-full items-center justify-center text-muted-foreground mt-20 space-y-4">
					<div class="h-12 w-12 rounded-full bg-muted flex items-center justify-center">
						<Send class="h-6 w-6 opacity-50" />
					</div>
					<p>This is the beginning of the channel.</p>
				</div>
			{/if}
		</div>
	</div>

	<!-- Input Area -->
	<div class="px-4 md:px-6 pb-6 pt-2 bg-background">
		{#if attachments.length > 0}
			<div class="mb-2 flex flex-wrap gap-2 p-2 bg-muted/20 rounded-md border border-dashed">
				{#each attachments as file (file.id)}
					<FilePreview {file} />
				{/each}
			</div>
		{/if}
		
		<div class="relative flex flex-col rounded-lg border bg-background shadow-sm focus-within:ring-1 focus-within:ring-primary/50 transition-all">
			<textarea
				bind:value={newMessage}
				placeholder="Message #general..."
				class="min-h-[60px] w-full resize-none bg-transparent px-4 py-3 text-sm focus:outline-none placeholder:text-muted-foreground scrollbar-thin"
				onkeydown={(e) => {
					if (e.key === 'Enter' && !e.shiftKey) {
						e.preventDefault();
						sendMessage();
					}
				}}
				disabled={isSending}
			></textarea>
			
			<div class="flex items-center justify-between p-2 pt-0">
				<div class="flex items-center gap-1">
					<FileUpload onUploadComplete={(file) => (attachments = [...attachments, file])} />
				</div>
				<Button size="sm" onclick={sendMessage} disabled={isSending || (!newMessage.trim() && attachments.length === 0)} class="h-8 gap-1.5 px-3">
					<Send class="h-3.5 w-3.5" />
					Send
				</Button>
			</div>
		</div>
		<div class="text-center mt-2 text-[10px] text-muted-foreground font-medium flex items-center justify-center gap-1">
			<span class="inline-block w-1.5 h-1.5 rounded-full bg-green-500"></span>
			WebSockets connected to Flora locally
		</div>
	</div>
</div>
