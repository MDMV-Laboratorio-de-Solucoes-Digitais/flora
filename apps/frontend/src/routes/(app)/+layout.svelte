<script lang="ts">
	import { authState } from '$lib/state/AuthState.svelte';
	import { Search, Menu, MessageSquare, Hash, CheckSquare, FileText, Settings, User, LogOut, ChevronLeft, ChevronRight, PanelRightClose, PanelRightOpen, Sparkles } from 'lucide-svelte';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';

	let { children } = $props();

	// App state
	let sidebarOpen = $state(true);
	let rightPanelOpen = $state(false);
</script>

<div class="flex h-screen w-full overflow-hidden bg-background">
	<!-- Sidebar -->
	<aside 
		class="bg-muted/30 flex shrink-0 flex-col border-r transition-all duration-300 ease-in-out z-20 {sidebarOpen ? 'w-64' : 'w-16'}"
	>
		<!-- Sidebar Header -->
		<div class="flex h-14 items-center justify-between border-b px-4">
			{#if sidebarOpen}
				<div class="flex items-center gap-2 font-bold text-primary truncate">
					<div class="h-6 w-6 rounded-md bg-primary text-primary-foreground flex items-center justify-center">F</div>
					<span>Flora Workspace</span>
				</div>
			{:else}
				<div class="mx-auto h-6 w-6 rounded-md bg-primary text-primary-foreground flex items-center justify-center font-bold">F</div>
			{/if}
		</div>

		<!-- Navigation -->
		<nav class="flex-1 overflow-y-auto py-4 scrollbar-thin">
			<ul class="space-y-1 px-2">
				<li>
					<a href="/channels" class="hover:bg-accent hover:text-accent-foreground flex items-center rounded-md px-3 py-2 text-sm font-medium transition-colors {sidebarOpen ? 'justify-start' : 'justify-center'}">
						<Hash class="h-4 w-4 {sidebarOpen ? 'mr-3' : ''} text-muted-foreground" />
						{#if sidebarOpen}<span>Channels</span>{/if}
					</a>
				</li>
				<li>
					<a href="/dms" class="hover:bg-accent hover:text-accent-foreground flex items-center rounded-md px-3 py-2 text-sm font-medium transition-colors {sidebarOpen ? 'justify-start' : 'justify-center'}">
						<MessageSquare class="h-4 w-4 {sidebarOpen ? 'mr-3' : ''} text-muted-foreground" />
						{#if sidebarOpen}<span>Direct Messages</span>{/if}
					</a>
				</li>
				<li>
					<a href="/tasks" class="hover:bg-accent hover:text-accent-foreground flex items-center rounded-md px-3 py-2 text-sm font-medium transition-colors {sidebarOpen ? 'justify-start' : 'justify-center'}">
						<CheckSquare class="h-4 w-4 {sidebarOpen ? 'mr-3' : ''} text-muted-foreground" />
						{#if sidebarOpen}<span>Tasks</span>{/if}
					</a>
				</li>
			</ul>
		</nav>

		<!-- Sidebar Footer -->
		<div class="border-t p-2">
			<Button variant="ghost" class="w-full {sidebarOpen ? 'justify-between' : 'justify-center'} px-3" onclick={() => sidebarOpen = !sidebarOpen}>
				{#if sidebarOpen}
					<span class="text-sm">Collapse</span>
					<ChevronLeft class="h-4 w-4" />
				{:else}
					<ChevronRight class="h-4 w-4" />
				{/if}
			</Button>
		</div>
	</aside>

	<!-- Main Content Area -->
	<div class="flex flex-1 flex-col overflow-hidden relative">
		<!-- Header -->
		<header class="flex h-14 shrink-0 items-center gap-4 border-b bg-background px-4 lg:px-6 z-10">
			<!-- Mobile sidebar toggle -->
			<Button variant="ghost" size="icon" class="md:hidden" onclick={() => sidebarOpen = !sidebarOpen}>
				<Menu class="h-5 w-5" />
				<span class="sr-only">Toggle Sidebar</span>
			</Button>

			<!-- Context Title -->
			<div class="flex items-center gap-2 font-semibold">
				<Hash class="h-4 w-4 text-muted-foreground" />
				<span>general</span>
			</div>

			<!-- Global Search -->
			<div class="flex-1 flex justify-center px-4 max-w-2xl mx-auto">
				<div class="relative w-full max-w-md hidden md:block">
					<Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
					<Input type="search" placeholder="Search across workspace..." class="w-full bg-muted/50 pl-9 pr-4 focus-visible:ring-primary h-9 rounded-full" />
				</div>
			</div>

			<!-- Right Actions -->
			<div class="flex items-center gap-2">
				<!-- Right Panel Toggle -->
				<Button variant="ghost" size="icon" onclick={() => rightPanelOpen = !rightPanelOpen} class="hidden sm:flex text-muted-foreground hover:text-foreground">
					{#if rightPanelOpen}
						<PanelRightClose class="h-5 w-5" />
					{:else}
						<PanelRightOpen class="h-5 w-5" />
					{/if}
				</Button>

				<!-- User Profile / Settings -->
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						<Button variant="ghost" size="icon" class="rounded-full overflow-hidden border">
							<User class="h-5 w-5" />
						</Button>
					</DropdownMenu.Trigger>
					<DropdownMenu.Content align="end">
						<DropdownMenu.Label>My Account</DropdownMenu.Label>
						<DropdownMenu.Separator />
						<DropdownMenu.Item><Settings class="mr-2 h-4 w-4" /> Settings</DropdownMenu.Item>
						<DropdownMenu.Separator />
						<DropdownMenu.Item onclick={() => authState.logout()} class="text-destructive focus:text-destructive"><LogOut class="mr-2 h-4 w-4" /> Logout</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
		</header>

		<!-- Content + Right Panel wrapper -->
		<div class="flex flex-1 overflow-hidden">
			<!-- Main Page Content -->
			<main class="flex-1 overflow-y-auto bg-background relative flex flex-col">
				{@render children()}
			</main>

			<!-- Right Panel (Contextual) -->
			{#if rightPanelOpen}
				<aside class="w-80 border-l bg-background shrink-0 overflow-y-auto hidden sm:block animate-in slide-in-from-right-8 duration-200">
					<div class="flex h-14 items-center justify-between border-b px-4">
						<h3 class="font-semibold text-sm">Thread Details</h3>
						<Button variant="ghost" size="icon" class="h-8 w-8" onclick={() => rightPanelOpen = false}>
							<PanelRightClose class="h-4 w-4" />
						</Button>
					</div>
					<div class="p-4 flex flex-col gap-4 text-sm">
						<!-- Placeholder content for context panel -->
						<div class="rounded-lg border p-3 bg-muted/20">
							<div class="flex items-center gap-2 mb-2 font-medium text-primary">
								<Sparkles class="h-4 w-4" />
								<span>AI Summary</span>
							</div>
							<p class="text-muted-foreground text-xs leading-relaxed">
								This feature uses an external API (Feature Flag: ON). The team discussed the recent VPS performance issues and agreed to migrate the database by next Friday.
							</p>
						</div>
						
						<p class="text-muted-foreground text-center mt-8">Select a message or task to view details here.</p>
					</div>
				</aside>
			{/if}
		</div>
	</div>
</div>
