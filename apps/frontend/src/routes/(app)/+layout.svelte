<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { authState } from '$lib/state/AuthState.svelte';
	import {
		Search,
		Menu,
		MessageSquare,
		Hash,
		CheckSquare,
		Settings,
		User,
		LogOut,
		ChevronLeft,
		ChevronRight,
		PanelRightClose,
		PanelRightOpen,
		Sparkles
	} from 'lucide-svelte';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';

	let { children } = $props();

	// App state
	let isSidebarOpen = $state(true);
	let isRightPanelOpen = $state(false);

	function enforceAuth() {
		if (typeof window === 'undefined') return;
		if (!authState.isAuthenticated || !authState.token) {
			void goto('/');
			return;
		}
		const path = page.url.pathname;
		if (!authState.organizationId && path !== '/select-org') {
			void goto('/select-org');
		}
	}

	onMount(() => {
		enforceAuth();
	});

	$effect(() => {
		page.url.pathname;
		authState.isAuthenticated;
		authState.token;
		authState.organizationId;
		enforceAuth();
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
					<span>Flora Workspace</span>
				</div>
			{:else}
				<div
					class="mx-auto flex h-6 w-6 items-center justify-center rounded-md bg-primary font-bold text-primary-foreground"
				>
					F
				</div>
			{/if}
		</div>

		<!-- Navigation -->
		<nav class="flex-1 scrollbar-thin overflow-y-auto py-4">
			<ul class="space-y-1 px-2">
				<li>
					<a
						href="/channels"
						class="flex items-center rounded-md px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground {isSidebarOpen
							? 'justify-start'
							: 'justify-center'}"
					>
						<Hash class="h-4 w-4 {isSidebarOpen ? 'mr-3' : ''} text-muted-foreground" />
						{#if isSidebarOpen}<span>Channels</span>{/if}
					</a>
				</li>
				<li>
					<a
						href="/dms"
						class="flex items-center rounded-md px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground {isSidebarOpen
							? 'justify-start'
							: 'justify-center'}"
					>
						<MessageSquare class="h-4 w-4 {isSidebarOpen ? 'mr-3' : ''} text-muted-foreground" />
						{#if isSidebarOpen}<span>Direct Messages</span>{/if}
					</a>
				</li>
				<li>
					<a
						href="/tasks"
						class="flex items-center rounded-md px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground {isSidebarOpen
							? 'justify-start'
							: 'justify-center'}"
					>
						<CheckSquare class="h-4 w-4 {isSidebarOpen ? 'mr-3' : ''} text-muted-foreground" />
						{#if isSidebarOpen}<span>Tasks</span>{/if}
					</a>
				</li>
			</ul>
		</nav>

		<!-- Sidebar Footer -->
		<div class="border-t p-2">
			<Button
				variant="ghost"
				class="w-full {isSidebarOpen ? 'justify-between' : 'justify-center'} px-3"
				onclick={() => (isSidebarOpen = !isSidebarOpen)}
			>
				{#if isSidebarOpen}
					<span class="text-sm">Collapse</span>
					<ChevronLeft class="h-4 w-4" />
				{:else}
					<ChevronRight class="h-4 w-4" />
				{/if}
			</Button>
		</div>
	</aside>

	<!-- Main Content Area -->
	<div class="relative flex flex-1 flex-col overflow-hidden">
		<!-- Header -->
		<header class="z-10 flex h-14 shrink-0 items-center gap-4 border-b bg-background px-4 lg:px-6">
			<!-- Mobile sidebar toggle -->
			<Button
				variant="ghost"
				size="icon"
				class="md:hidden"
				onclick={() => (isSidebarOpen = !isSidebarOpen)}
			>
				<Menu class="h-5 w-5" />
				<span class="sr-only">Toggle Sidebar</span>
			</Button>

			<!-- Context Title -->
			<div class="flex items-center gap-2 font-semibold">
				<Hash class="h-4 w-4 text-muted-foreground" />
				<span>general</span>
			</div>

			<!-- Global Search -->
			<div class="mx-auto flex max-w-2xl flex-1 justify-center px-4">
				<div class="relative hidden w-full max-w-md md:block">
					<Search class="absolute top-2.5 left-2.5 h-4 w-4 text-muted-foreground" />
					<Input
						type="search"
						placeholder="Search across workspace..."
						class="h-9 w-full rounded-full bg-muted/50 pr-4 pl-9 focus-visible:ring-primary"
					/>
				</div>
			</div>

			<!-- Right Actions -->
			<div class="flex items-center gap-2">
				<!-- Right Panel Toggle -->
				<Button
					variant="ghost"
					size="icon"
					onclick={() => (isRightPanelOpen = !isRightPanelOpen)}
					class="hidden text-muted-foreground hover:text-foreground sm:flex"
				>
					{#if isRightPanelOpen}
						<PanelRightClose class="h-5 w-5" />
					{:else}
						<PanelRightOpen class="h-5 w-5" />
					{/if}
				</Button>

				<!-- User Profile / Settings -->
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						<Button variant="ghost" size="icon" class="overflow-hidden rounded-full border">
							<User class="h-5 w-5" />
						</Button>
					</DropdownMenu.Trigger>
					<DropdownMenu.Content align="end">
						<DropdownMenu.Label>My Account</DropdownMenu.Label>
						<DropdownMenu.Separator />
						<DropdownMenu.Item><Settings class="mr-2 h-4 w-4" /> Settings</DropdownMenu.Item>
						<DropdownMenu.Separator />
						<DropdownMenu.Item
							onclick={() => authState.logout()}
							class="text-destructive focus:text-destructive"
							><LogOut class="mr-2 h-4 w-4" /> Logout</DropdownMenu.Item
						>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
		</header>

		<!-- Content + Right Panel wrapper -->
		<div class="flex flex-1 overflow-hidden">
			<!-- Main Page Content -->
			<main class="relative flex flex-1 flex-col overflow-y-auto bg-background">
				{@render children()}
			</main>

			<!-- Right Panel (Contextual) -->
			{#if isRightPanelOpen}
				<aside
					class="animate-in slide-in-from-right-8 hidden w-80 shrink-0 overflow-y-auto border-l bg-background duration-200 sm:block"
				>
					<div class="flex h-14 items-center justify-between border-b px-4">
						<h3 class="text-sm font-semibold">Thread Details</h3>
						<Button
							variant="ghost"
							size="icon"
							class="h-8 w-8"
							onclick={() => (isRightPanelOpen = false)}
						>
							<PanelRightClose class="h-4 w-4" />
						</Button>
					</div>
					<div class="flex flex-col gap-4 p-4 text-sm">
						<!-- Placeholder content for context panel -->
						<div class="rounded-lg border bg-muted/20 p-3">
							<div class="mb-2 flex items-center gap-2 font-medium text-primary">
								<Sparkles class="h-4 w-4" />
								<span>AI Summary</span>
							</div>
							<p class="text-xs leading-relaxed text-muted-foreground">
								This feature uses an external API (Feature Flag: ON). The team discussed the recent
								VPS performance issues and agreed to migrate the database by next Friday.
							</p>
						</div>

						<p class="mt-8 text-center text-muted-foreground">
							Select a message or task to view details here.
						</p>
					</div>
				</aside>
			{/if}
		</div>
	</div>
</div>
