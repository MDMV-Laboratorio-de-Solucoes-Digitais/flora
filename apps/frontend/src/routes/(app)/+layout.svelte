<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { ApiClient } from '$lib/api/client';
	import { toNotification, type ApiNotification } from '$lib/api/mappers';
	import { authState } from '$lib/state/AuthState.svelte';
	import { notificationStore } from '$lib/state/NotificationStore.svelte';
	import { wsClient } from '$lib/api/websocket';
	import { logger } from '$lib/utils/logger';
	import { notificationsQueryPath } from '$lib/utils/notifications';
	import {
		Menu,
		Bell,
		Hash,
		CheckSquare,
		Settings,
		User,
		LogOut,
		ChevronLeft,
		Files
	} from '@lucide/svelte';
	import SearchBox from '$lib/components/search/SearchBox.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { format, t } from '$lib/i18n';
	import { emailFromToken } from '$lib/utils/jwt';

	let { children } = $props();
	const userEmail = $derived(emailFromToken(authState.token) ?? t['nav.userEmail']);

	// App state
	let isSidebarOpen = $state(true);
	let wsKey = $state<string | null>(null);
	let notifKey = $state<string | null>(null);
	let notifLoadSeq = 0;

	function enforceAuth() {
		if (globalThis.window === undefined) return;
		if (!authState.isAuthenticated || !authState.token) {
			void goto('/');
			return;
		}
		const path = page.url.pathname;
		if (path !== '/select-org' && !authState.organizationId) {
			void goto('/select-org');
		}
	}

	function disconnectWs() {
		wsClient.disconnect();
	}

	async function preloadNotifications(key: string) {
		const seq = ++notifLoadSeq;
		try {
			const response = await ApiClient.request<ApiNotification[]>(
				notificationsQueryPath({ limit: 50 })
			);
			if (seq !== notifLoadSeq || notifKey !== key) return;
			notificationStore.setNotifications(response.map((item) => toNotification(item)));
		} catch (error) {
			if (seq !== notifLoadSeq) return;
			logger.warn('notifications', 'Failed to preload notifications', {
				message: error instanceof Error ? error.message : String(error)
			});
		}
	}

	onMount(() => {
		enforceAuth();
		return disconnectWs;
	});

	$effect(() => {
		void page.url.pathname;
		void authState.isAuthenticated;
		void authState.token;
		void authState.organizationId;
		enforceAuth();
	});

	$effect(() => {
		const key =
			authState.isAuthenticated &&
			authState.token &&
			authState.organizationId &&
			!authState.isSessionLocked &&
			!authState.isInvalidated
				? `${authState.token}|${authState.organizationId}`
				: null;

		if (!key) {
			wsKey = null;
			wsClient.disconnect();
		} else if (key !== wsKey) {
			wsClient.disconnect();
			wsKey = key;
			wsClient.connect();
		}

		const nextNotifKey =
			authState.isAuthenticated &&
			authState.token &&
			authState.organizationId &&
			!authState.isSessionLocked &&
			!authState.isInvalidated
				? `${authState.organizationId}|${authState.token}`
				: null;

		if (!nextNotifKey) {
			notifKey = null;
			notifLoadSeq += 1;
			notificationStore.clear();
		} else if (nextNotifKey !== notifKey) {
			notifKey = nextNotifKey;
			notificationStore.clear();
			void preloadNotifications(nextNotifKey);
		}
	});
</script>

<div class="flex h-screen w-full overflow-hidden bg-background">
	<!-- Sidebar -->
	<aside
		class="z-20 flex shrink-0 flex-col border-r bg-muted/30 transition-all duration-300 ease-in-out {isSidebarOpen
			? 'w-64'
			: 'w-16'}"
	>
		<!-- Sidebar Header -->
		<div class="flex h-14 items-center justify-between border-b px-4">
			{#if isSidebarOpen}
				<div class="flex items-center gap-2 truncate font-bold text-foreground">
					<div
						class="flex h-6 w-6 items-center justify-center rounded-md bg-primary text-primary-foreground"
					>
						F
					</div>
					<span>{t['app.name']}</span>
				</div>
			{/if}
			<Button
				variant="ghost"
				size="icon"
				aria-label={isSidebarOpen ? t['nav.closeSidebar'] : t['nav.openSidebar']}
				class="h-8 w-8 shrink-0"
				onclick={() => (isSidebarOpen = !isSidebarOpen)}
			>
				{#if isSidebarOpen}
					<ChevronLeft class="h-4 w-4" />
				{:else}
					<Menu class="h-4 w-4" />
				{/if}
			</Button>
		</div>

		<!-- Sidebar Content -->
		<div class="flex-1 space-y-4 overflow-y-auto py-4">
			<div class="px-3">
				<div class="space-y-1">
					<Button
						variant="ghost"
						class="w-full justify-start {isSidebarOpen ? 'px-3' : 'justify-center px-0'}"
						href="/channels"
					>
						<Hash class="h-4 w-4 {isSidebarOpen ? 'mr-2' : ''}" />
						{#if isSidebarOpen}<span>{t['nav.channels']}</span>{/if}
					</Button>
					<Button
						variant="ghost"
						class="w-full justify-start {isSidebarOpen ? 'px-3' : 'justify-center px-0'}"
						href="/tasks"
					>
						<CheckSquare class="h-4 w-4 {isSidebarOpen ? 'mr-2' : ''}" />
						{#if isSidebarOpen}<span>{t['nav.tasks']}</span>{/if}
					</Button>
					<Button
						variant="ghost"
						class="w-full justify-start {isSidebarOpen ? 'px-3' : 'justify-center px-0'}"
						href="/files"
						aria-label={t['nav.files']}
					>
						<Files class="h-4 w-4 {isSidebarOpen ? 'mr-2' : ''}" />
						{#if isSidebarOpen}<span>{t['nav.files']}</span>{/if}
					</Button>
					<Button
						variant="ghost"
						class="w-full justify-start {isSidebarOpen ? 'px-3' : 'justify-center px-0'}"
						href="/notifications"
						aria-label={t['nav.notifications']}
					>
						<Bell class="h-4 w-4 {isSidebarOpen ? 'mr-2' : ''}" />
						{#if isSidebarOpen}
							<span>{t['nav.notifications']}</span>
							{#if notificationStore.unreadCount > 0}
								<Badge
									variant="secondary"
									class="ml-auto"
									title={format('notifications.unreadCount', {
										count: notificationStore.unreadCount
									})}
								>
									{notificationStore.unreadCount}
								</Badge>
							{/if}
						{/if}
					</Button>
				</div>
			</div>

			{#if isSidebarOpen}
				<div class="px-3">
					<h3
						class="mb-2 px-3 text-xs font-semibold tracking-wider text-muted-foreground uppercase"
					>
						{t['nav.channels']}
					</h3>
					<p class="px-3 text-xs text-muted-foreground">
						{t['nav.channelsHint']}
					</p>
				</div>
			{/if}
		</div>

		<!-- Sidebar Footer -->
		<div class="mt-auto border-t p-3">
			<DropdownMenu.Root>
				<DropdownMenu.Trigger>
					{#snippet child({ props })}
						<Button
							{...props}
							variant="ghost"
							class="w-full justify-start {isSidebarOpen ? 'px-2' : 'justify-center px-0'}"
						>
							<div
								class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary"
							>
								<User class="h-4 w-4" />
							</div>
							{#if isSidebarOpen}
								<div class="ml-2 flex flex-col items-start overflow-hidden text-sm">
									<span class="w-full truncate font-medium"
										>{userEmail === t['nav.userEmail']
											? t['nav.userName']
											: userEmail.split('@', 1)[0]}</span
									>
									<span class="w-full truncate text-xs text-muted-foreground">{userEmail}</span>
								</div>
							{/if}
						</Button>
					{/snippet}
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end" class="w-56">
					<DropdownMenu.Label>{t['nav.myAccount']}</DropdownMenu.Label>
					<DropdownMenu.Separator />
					<DropdownMenu.Item>
						{#snippet child({ props })}
							<a href="/settings" {...props}>
								<Settings class="mr-2 h-4 w-4" />
								<span>{t['nav.settings']}</span>
							</a>
						{/snippet}
					</DropdownMenu.Item>
					<DropdownMenu.Item
						onclick={() => {
							authState.logout();
							void goto('/');
						}}
					>
						<LogOut class="mr-2 h-4 w-4" />
						<span>{t['nav.logout']}</span>
					</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		</div>
	</aside>

	<!-- Main Content Wrapper -->
	<div class="flex h-full min-w-0 flex-1 flex-col overflow-hidden">
		<!-- Header -->
		<header
			class="z-10 flex h-14 shrink-0 items-center justify-between border-b bg-background px-4"
		>
			<div class="flex flex-1 items-center gap-4">
				<SearchBox />
			</div>
		</header>

		<!-- Main Split Area -->
		<div class="flex flex-1 overflow-hidden">
			<main class="relative flex-1 overflow-auto">
				{@render children?.()}
			</main>
		</div>
	</div>
</div>
