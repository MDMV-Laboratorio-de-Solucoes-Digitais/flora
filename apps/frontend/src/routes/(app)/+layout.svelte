<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { authState } from '$lib/state/AuthState.svelte';
	import { wsClient } from '$lib/api/websocket';
	import {
		Menu,
		MessageSquare,
		Hash,
		CheckSquare,
		Settings,
		User,
		LogOut,
		ChevronLeft,
		PanelRightClose,
		PanelRightOpen,
		Sparkles
	} from '@lucide/svelte';
	import { Input } from '$lib/components/ui/input';
	import SearchBox from '$lib/components/search/SearchBox.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { t } from '$lib/i18n';
	import { emailFromToken } from '$lib/utils/jwt';

	let { children } = $props();
	const userEmail = $derived(emailFromToken(authState.token) ?? t['nav.userEmail']);

	// App state
	let isSidebarOpen = $state(true);
	let isRightPanelOpen = $state(false);

	let wsKey = $state<string | null>(null);

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
			return;
		}

		if (key !== wsKey) {
			wsClient.disconnect();
			wsKey = key;
			wsClient.connect();
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
				<div class="flex items-center gap-2 truncate font-bold text-primary">
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
						href="/dms"
					>
						<MessageSquare class="h-4 w-4 {isSidebarOpen ? 'mr-2' : ''}" />
						{#if isSidebarOpen}<span>{t['nav.directMessages']}</span>{/if}
					</Button>
					<Button
						variant="ghost"
						class="w-full justify-start {isSidebarOpen ? 'px-3' : 'justify-center px-0'}"
						href="/tasks"
					>
						<CheckSquare class="h-4 w-4 {isSidebarOpen ? 'mr-2' : ''}" />
						{#if isSidebarOpen}<span>{t['nav.tasks']}</span>{/if}
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

			<div class="ml-4 flex items-center gap-2">
				<Button
					variant="ghost"
					size="icon"
					onclick={() => (isRightPanelOpen = !isRightPanelOpen)}
					title={isRightPanelOpen ? t['nav.closeAiPanel'] : t['nav.openAiPanel']}
					class={isRightPanelOpen ? 'bg-muted text-primary' : ''}
				>
					{#if isRightPanelOpen}
						<PanelRightClose class="h-5 w-5" />
					{:else}
						<Sparkles class="h-5 w-5 text-primary" />
					{/if}
				</Button>
			</div>
		</header>

		<!-- Main Split Area -->
		<div class="flex flex-1 overflow-hidden">
			<!-- View Container -->
			<main class="relative flex-1 overflow-auto">
				{@render children?.()}
			</main>

			<!-- AI Insight Panel -->
			{#if isRightPanelOpen}
				<aside
					class="z-10 flex w-80 shrink-0 flex-col overflow-hidden border-l bg-muted/10 shadow-xl"
				>
					<div class="flex h-12 items-center justify-between border-b bg-muted/30 px-4">
						<div class="flex items-center gap-2 text-sm font-semibold text-primary">
							<Sparkles class="h-4 w-4" />
							{t['ai.panelTitle']}
						</div>
						<Button
							variant="ghost"
							size="icon"
							class="h-6 w-6"
							onclick={() => (isRightPanelOpen = false)}
						>
							<PanelRightOpen class="h-4 w-4" />
						</Button>
					</div>
					<div class="flex flex-1 flex-col items-center justify-center p-4 text-center">
						<div class="mb-4 rounded-full bg-primary/10 p-3">
							<Sparkles class="h-6 w-6 text-primary" />
						</div>
						<h3 class="mb-1 text-sm font-medium">{t['ai.welcome']}</h3>
						<p class="mb-4 text-xs text-muted-foreground">
							{t['ai.welcomeHint']}
						</p>
						<div class="w-full">
							<Input placeholder={t['ai.askPlaceholder']} class="w-full" />
						</div>
					</div>
				</aside>
			{/if}
		</div>
	</div>
</div>
