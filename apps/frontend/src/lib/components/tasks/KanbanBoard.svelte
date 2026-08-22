<script lang="ts">
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Avatar, AvatarFallback } from '$lib/components/ui/avatar';
	
	// Basic Kanban state
	let columns = $state([
		{ id: 'todo', title: 'To Do', tasks: [
			{ id: 't1', title: 'Setup DB', desc: 'Initialize PostgreSQL with docker-compose', tags: ['Backend'] },
			{ id: 't2', title: 'Configure Vite', desc: 'Add Tailwind and shadcn-svelte', tags: ['Frontend'] }
		]},
		{ id: 'in-progress', title: 'In Progress', tasks: [
			{ id: 't3', title: 'Kanban UI', desc: 'Implement drag and drop', tags: ['Frontend', 'UI/UX'] }
		]},
		{ id: 'done', title: 'Done', tasks: [
			{ id: 't4', title: 'Project Scaffolding', desc: 'Create monorepo structure', tags: ['DevOps'] }
		]}
	]);

	let draggedItem = $state<any>(null);
	let draggedFromCol = $state<string | null>(null);

	function handleDragStart(event: DragEvent, task: any, colId: string) {
		draggedItem = task;
		draggedFromCol = colId;
		if (event.dataTransfer) {
			event.dataTransfer.effectAllowed = 'move';
			// Firefox requires data to be set
			event.dataTransfer.setData('text/plain', task.id);
		}
	}

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
		if (event.dataTransfer) {
			event.dataTransfer.dropEffect = 'move';
		}
	}

	function handleDrop(event: DragEvent, targetColId: string) {
		event.preventDefault();
		if (!draggedItem || !draggedFromCol) return;
		if (draggedFromCol === targetColId) return;

		// Move item between columns
		columns = columns.map(col => {
			if (col.id === draggedFromCol) {
				return { ...col, tasks: col.tasks.filter(t => t.id !== draggedItem.id) };
			}
			if (col.id === targetColId) {
				return { ...col, tasks: [...col.tasks, draggedItem] };
			}
			return col;
		});

		draggedItem = null;
		draggedFromCol = null;
	}
</script>

<div class="flex h-full w-full gap-4 overflow-x-auto p-4 scrollbar-thin">
	{#each columns as col}
		<div 
			class="flex h-full w-80 shrink-0 flex-col rounded-xl bg-muted/40 p-3"
			ondragover={handleDragOver}
			ondrop={(e) => handleDrop(e, col.id)}
			role="region"
			aria-label="{col.title} column"
		>
			<div class="mb-3 flex items-center justify-between px-1">
				<h3 class="font-semibold">{col.title}</h3>
				<span class="rounded-full bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">{col.tasks.length}</span>
			</div>
			
			<div class="flex flex-1 flex-col gap-2 overflow-y-auto min-h-[150px]">
				{#each col.tasks as task}
					<div
						role="button"
						tabindex="0"
						draggable="true"
						ondragstart={(e) => handleDragStart(e, task, col.id)}
						class="cursor-grab active:cursor-grabbing hover:ring-2 hover:ring-primary/50 transition-all rounded-lg"
					>
						<Card class="shadow-sm">
							<CardHeader class="p-3 pb-0">
								<CardTitle class="text-sm font-medium">{task.title}</CardTitle>
							</CardHeader>
							<CardContent class="p-3 pt-2">
								<p class="text-xs text-muted-foreground mb-3">{task.desc}</p>
								<div class="flex items-center justify-between mt-auto">
									<div class="flex flex-wrap gap-1">
										{#each task.tags as tag}
											<Badge variant="secondary" class="text-[10px] px-1 py-0 h-4">{tag}</Badge>
										{/each}
									</div>
									<div class="h-5 w-5 rounded-full bg-primary/20 flex items-center justify-center text-[10px] font-bold text-primary shrink-0">
										U
									</div>
								</div>
							</CardContent>
						</Card>
					</div>
				{/each}
			</div>
		</div>
	{/each}
</div>
