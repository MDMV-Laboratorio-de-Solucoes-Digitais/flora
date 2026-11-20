<script lang="ts">
	import { messageStore } from '$lib/state/MessageStore.svelte';
	import { ApiClient } from '$lib/api/client';
	import { toMessage, type ApiMessage } from '$lib/api/mappers';
	import { wsClient } from '$lib/api/websocket';
	import { logger } from '$lib/utils/logger';
	import { Button } from '$lib/components/ui/button';
	import ThreadReply from './thread/ThreadReply.svelte';
	import { tick } from 'svelte';
	import { Send, Sparkles } from '@lucide/svelte';

	let { channelId = '' } = $props<{ channelId?: string }>();

	let newMessage = $state('');
	let isSending = $state(false);
	let isLoading = $state(false);
	let loadError = $state('');
	let chatContainer = $state<HTMLElement>();

	async function scrollToBottom() {
		await tick();
		if (chatContainer) {
			chatContainer.scrollTop = chatContainer.scrollHeight;
		}
	}

	function updateLastKnownFromStore() {
		const msgs = messageStore.messages;
		wsClient.lastKnownMessageId = msgs.at(-1)?.id ?? null;
	}

	async function loadMessages(id: string) {
		if (!id) {
			messageStore.clear();
			wsClient.setActiveChannel(null);
			wsClient.lastKnownMessageId = null;
			return;
		}
		isLoading = true;
		loadError = '';
		try {
			const messages = await ApiClient.request<ApiMessage[]>(
				`/api/v1/channels/${id}/messages?limit=100`
			);
			// API returns newest-first by default; display chronologically.
			messageStore.setMessages(messages.toReversed().map((message) => toMessage(message)));
			updateLastKnownFromStore();
			wsClient.connect();
			wsClient.setActiveChannel(id);
			await scrollToBottom();
		} catch (error) {
			loadError = error instanceof Error ? error.message : 'Failed to load messages';
			logger.error('chat', 'Failed to fetch messages', { channelId: id }, error as Error);
			messageStore.clear();
			wsClient.lastKnownMessageId = null;
		} finally {
			isLoading = false;
		}
	}

	$effect(() => {
		void loadMessages(channelId);
	});

	async function sendMessage() {
		if (!channelId || !newMessage.trim()) return;
		isSending = true;
		try {
			const created = await ApiClient.request<ApiMessage>(
				`/api/v1/channels/${channelId}/messages`,
				{
					method: 'POST',
					body: JSON.stringify({ content: newMessage.trim() })
				}
			);
			const mapped = toMessage(created);
			messageStore.upsertMessage(mapped);
			wsClient.lastKnownMessageId = mapped.id;
			newMessage = '';
			await scrollToBottom();
		} catch (error) {
			loadError = error instanceof Error ? error.message : 'Failed to send message';
			logger.error('chat', 'Failed to send message', { channelId }, error as Error);
		} finally {
			isSending = false;
		}
	}

	async function onReplySent(createdApi: ApiMessage) {
		const mapped = toMessage(createdApi);
		messageStore.upsertMessage(mapped);
		wsClient.lastKnownMessageId = mapped.id;
		await scrollToBottom();
	}
</script>

<div class="relative flex h-full flex-col overflow-hidden bg-background">
	<div class="flex-1 overflow-y-auto" bind:this={chatContainer}>
		<div class="py-4 pb-8">
			{#if !channelId}
				<div
					class="mt-20 flex h-full flex-col items-center justify-center space-y-3 text-muted-foreground"
				>
					<p>Select a channel to view messages.</p>
				</div>
			{:else if isLoading}
				<div class="mt-20 flex h-full items-center justify-center text-muted-foreground">
					Loading messages…
				</div>
			{:else if loadError}
				<div
					class="mx-6 mt-6 rounded-md border border-destructive/30 bg-destructive/10 p-3 text-sm text-destructive"
				>
					{loadError}
				</div>
			{:else}
				{#each messageStore.messages.filter((m) => !m.threadId) as message (message.id)}
					<div class="group flex flex-col px-4 py-2 transition-colors hover:bg-muted/40 md:px-6">
						<div class="flex gap-4">
							<div
								class="flex h-10 w-10 shrink-0 items-center justify-center rounded-md border bg-secondary font-bold text-secondary-foreground"
							>
								{(message.authorId.at(0) ?? '?').toUpperCase()}
							</div>

							<div class="min-w-0 flex-1">
								<div class="mb-1 flex items-baseline space-x-2">
									<span class="text-[15px] font-semibold text-foreground"
										>User {message.authorId.slice(0, 8)}</span
									>
									<span class="text-xs text-muted-foreground"
										>{new Date(message.createdAt).toLocaleTimeString([], {
											hour: '2-digit',
											minute: '2-digit'
										})}</span
									>
									{#if message.content.includes('[AI]')}
										<span
											class="inline-flex items-center gap-1 rounded bg-purple-100 px-1.5 py-0.5 text-[10px] font-medium text-purple-800 dark:bg-purple-900/30 dark:text-purple-300"
										>
											<Sparkles class="h-3 w-3" /> AI
										</span>
									{/if}
								</div>

								<div
									class="font-sans text-sm leading-relaxed whitespace-pre-wrap text-foreground/90"
								>
									{message.content.replace('[AI]', '')}
								</div>

								{#if messageStore.messages.some((m) => m.threadId === message.id)}
									<div class="mt-3 space-y-3 border-l-2 border-primary/20 pl-2">
										{#each messageStore.messages.filter((m) => m.threadId === message.id) as reply (reply.id)}
											<div class="flex gap-3">
												<div
													class="flex h-6 w-6 shrink-0 items-center justify-center rounded bg-secondary text-[10px] font-bold"
												>
													{(reply.authorId.at(0) ?? '?').toUpperCase()}
												</div>
												<div class="flex-1">
													<div class="flex items-baseline space-x-2">
														<span class="text-sm font-semibold"
															>User {reply.authorId.slice(0, 8)}</span
														>
														<span class="text-[10px] text-muted-foreground"
															>{new Date(reply.createdAt).toLocaleTimeString([], {
																hour: '2-digit',
																minute: '2-digit'
															})}</span
														>
													</div>
													<div class="mt-0.5 text-sm text-foreground/80">{reply.content}</div>
												</div>
											</div>
										{/each}
									</div>
								{/if}

								<div class="mt-2 opacity-0 transition-opacity group-hover:opacity-100">
									<ThreadReply parentMessageId={message.id} {channelId} onSent={onReplySent} />
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
						<p>No messages yet.</p>
					</div>
				{/if}
			{/if}
		</div>
	</div>

	<div class="bg-background px-4 pt-2 pb-6 md:px-6">
		<div
			class="relative flex flex-col rounded-lg border bg-background shadow-sm transition-all focus-within:ring-1 focus-within:ring-primary/50"
		>
			<textarea
				bind:value={newMessage}
				placeholder={channelId ? 'Write a message…' : 'Select a channel first'}
				class="min-h-[60px] w-full resize-none scrollbar-thin bg-transparent px-4 py-3 text-sm placeholder:text-muted-foreground focus:outline-none"
				onkeydown={(event) => {
					if (event.key !== 'Enter' || event.shiftKey) return;
					event.preventDefault();
					sendMessage();
				}}
				disabled={isSending || !channelId}></textarea>

			<div class="flex items-center justify-end p-2 pt-0">
				<Button
					size="sm"
					onclick={sendMessage}
					disabled={isSending || !channelId || !newMessage.trim()}
					class="h-8 gap-1.5 px-3"
				>
					<Send class="h-3.5 w-3.5" />
					Send
				</Button>
			</div>
		</div>
	</div>
</div>
