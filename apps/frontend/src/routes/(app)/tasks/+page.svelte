<script lang="ts">
	import KanbanBoard from '$lib/components/tasks/KanbanBoard.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Plus } from 'lucide-svelte';
</script>

<div class="flex h-full flex-col">
	<div class="flex items-center justify-between border-b px-6 py-4">
		<div>
			<h1 class="text-2xl font-bold tracking-tight">Tasks</h1>
			<p class="text-muted-foreground text-sm">Manage your team's work in one place.</p>
		</div>
		<Button>
			<Plus class="mr-2 h-4 w-4" />
			New Task
		</Button>
	</div>
	
	<div class="flex-1 overflow-hidden bg-background">
		<KanbanBoard />
	</div>
</div>
