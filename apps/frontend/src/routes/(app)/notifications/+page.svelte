<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { ApiClient } from '$lib/api/client';
	import { toNotification, type ApiNotification } from '$lib/api/mappers';
	import { authState } from '$lib/state/AuthState.svelte';
	import { notificationStore } from '$lib/state/NotificationStore.svelte';
	import {
		notificationEventLabel,
		notificationsQueryPath,
		resolveNotificationHref
	} from '$lib/utils/notifications';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { apiErrorMessage, t } from '$lib/i18n';

	let filterMode = $state<'all' | 'unread'>('all');
	let isLoading = $state(true);
	let isMutating = $state(false);
	let errorMessage = $state('');
	let loadSeq = 0;

	const visibleNotifications = $derived(
		filterMode === 'unread'
			? notificationStore.notifications.filter((item) => !item.isRead)
			: notificationStore.notifications
	);

	async function loadNotifications() {
		const seq = ++loadSeq;
		isLoading = true;
		errorMessage = '';
		try {
			const response = await ApiClient.request<ApiNotification[]>(
				notificationsQueryPath({ limit: 50 })
			);
			if (seq !== loadSeq) return;
			notificationStore.setNotifications(response.map((item) => toNotification(item)));
		} catch (error) {
			if (seq !== loadSeq) return;
			errorMessage = apiErrorMessage(error, 'error.loadNotifications');
		} finally {
			if (seq === loadSeq) {
				isLoading = false;
			}
		}
	}

	async function markOneAsRead(notificationId: string) {
		if (isMutating) return;
		isMutating = true;
		errorMessage = '';
		try {
			await ApiClient.request(`/api/v1/notifications/${notificationId}/read`, {
				method: 'POST'
			});
			notificationStore.markAsRead(notificationId);
		} catch (error) {
			errorMessage = apiErrorMessage(error, 'error.markNotificationRead');
		} finally {
			isMutating = false;
		}
	}

	async function markAllAsRead() {
		if (isMutating || notificationStore.unreadCount === 0) return;
		isMutating = true;
		errorMessage = '';
		try {
			await ApiClient.request('/api/v1/notifications/read-all', {
				method: 'POST'
			});
			notificationStore.markAllAsRead();
		} catch (error) {
			errorMessage = apiErrorMessage(error, 'error.markNotificationRead');
		} finally {
			isMutating = false;
		}
	}

	onMount(async () => {
		if (!authState.isAuthenticated || !authState.token) {
			await goto('/');
			return;
		}
		if (!authState.organizationId) {
			await goto('/select-org');
			return;
		}
		await loadNotifications();
	});
</script>

<div class="mx-auto flex w-full max-w-4xl flex-col gap-6 p-6">
	<div class="flex flex-wrap items-start justify-between gap-4">
		<div>
			<h1 class="text-2xl font-bold">{t['notifications.title']}</h1>
			<p class="text-sm text-muted-foreground">{t['notifications.description']}</p>
		</div>

		<div class="flex flex-wrap items-center gap-2">
			<Button variant="outline" size="sm" onclick={loadNotifications} disabled={isLoading}>
				{t['notifications.refresh']}
			</Button>
			<Button
				variant="secondary"
				size="sm"
				onclick={markAllAsRead}
				disabled={isLoading || isMutating || notificationStore.unreadCount === 0}
			>
				{t['notifications.markAllRead']}
			</Button>
		</div>
	</div>

	<div class="flex items-center gap-2" role="group" aria-label={t['notifications.title']}>
		<Button
			variant={filterMode === 'all' ? 'secondary' : 'outline'}
			size="sm"
			aria-pressed={filterMode === 'all'}
			onclick={() => (filterMode = 'all')}
		>
			{t['notifications.filterAll']}
		</Button>
		<Button
			variant={filterMode === 'unread' ? 'secondary' : 'outline'}
			size="sm"
			aria-pressed={filterMode === 'unread'}
			onclick={() => (filterMode = 'unread')}
		>
			{t['notifications.filterUnread']}
		</Button>
	</div>

	{#if errorMessage}
		<div
			role="alert"
			class="rounded-md border border-destructive/30 bg-destructive/5 p-3 text-sm text-destructive"
		>
			{errorMessage}
		</div>
	{/if}

	{#if isLoading}
		<p class="text-sm text-muted-foreground">{t['notifications.loading']}</p>
	{:else if visibleNotifications.length === 0}
		<div class="rounded-lg border p-6 text-sm text-muted-foreground">
			{filterMode === 'unread' ? t['notifications.emptyUnread'] : t['notifications.empty']}
		</div>
	{:else}
		<ul class="space-y-3" aria-label={t['notifications.title']}>
			{#each visibleNotifications as item (item.id)}
				<li
					class="flex flex-wrap items-center justify-between gap-4 rounded-lg border p-4 {item.isRead
						? 'bg-background'
						: 'border-primary/40 bg-muted/20'}"
				>
					<div class="min-w-0 flex-1 space-y-1">
						<div class="flex flex-wrap items-center gap-2">
							<Badge variant="secondary">{notificationEventLabel(item.eventType)}</Badge>
							<Badge variant="outline">
								{item.isRead ? t['notifications.read'] : t['notifications.unread']}
							</Badge>
							<span class="text-xs text-muted-foreground">
								{new Date(item.createdAt).toLocaleString('pt-BR')}
							</span>
						</div>
						<p class="truncate text-xs text-muted-foreground">{item.targetId}</p>
					</div>

					<div class="flex items-center gap-2">
						<Button variant="outline" size="sm" href={resolveNotificationHref(item)}>
							{t['notifications.openTarget']}
						</Button>
						{#if !item.isRead}
							<Button
								variant="secondary"
								size="sm"
								disabled={isMutating}
								onclick={() => markOneAsRead(item.id)}
							>
								{t['notifications.markRead']}
							</Button>
						{/if}
					</div>
				</li>
			{/each}
		</ul>
	{/if}
</div>
