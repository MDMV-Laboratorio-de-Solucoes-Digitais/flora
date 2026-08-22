<script lang="ts">
	import { authState } from '$lib/state/AuthState.svelte';
	let { children } = $props();
</script>

<div class="flex h-screen w-full overflow-hidden">
	<!-- Sidebar -->
	<aside class="bg-muted/50 flex w-64 shrink-0 flex-col border-r">
		<div class="border-b p-4 text-lg font-bold">Flora Workspace</div>
		<nav class="flex-1 overflow-y-auto py-4">
			<ul class="space-y-1 px-2">
				<li>
					<a
						href="/channels"
						class="hover:bg-accent hover:text-accent-foreground block rounded-md px-3 py-2 text-sm font-medium"
						>Channels</a
					>
				</li>
				<li>
					<a
						href="/dms"
						class="hover:bg-accent hover:text-accent-foreground block rounded-md px-3 py-2 text-sm font-medium"
						>Direct Messages</a
					>
				</li>
				<li>
					<a
						href="/tasks"
						class="hover:bg-accent hover:text-accent-foreground block rounded-md px-3 py-2 text-sm font-medium"
						>Tasks</a
					>
				</li>
				<li>
					<a
						href="/settings"
						class="hover:bg-accent hover:text-accent-foreground block rounded-md px-3 py-2 text-sm font-medium"
						>Settings</a
					>
				</li>
			</ul>
		</nav>
		<div class="border-t p-4">
			<button
				class="text-sm font-medium text-red-500 hover:underline"
				onclick={() => authState.logout()}>Logout</button
			>
		</div>
	</aside>

	<!-- Main Content -->
	<main class="flex-1 overflow-y-auto">
		{@render children()}
	</main>
</div>
