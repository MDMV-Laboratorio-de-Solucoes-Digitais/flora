<script lang="ts">
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';

	// Basic Kanban state
	let columns = $state([
		{
			id: 'todo',
			title: 'To Do',
			tasks: [
				{
					id: 't1',
					title: 'Setup DB',
					desc: 'Initialize PostgreSQL with docker-compose',
					tags: ['Backend']
				},
				{
					id: 't2',
					title: 'Configure Vite',
					desc: 'Add Tailwind and shadcn-svelte',
					tags: ['Frontend']
				}
			]
		},
		{
			id: 'in-progress',
			title: 'In Progress',
			tasks: [
				{
					id: 't3',
					title: 'Kanban UI',
					desc: 'Implement drag and drop',
					tags: ['Frontend', 'UI/UX']
				}
			]
		},
		{
			id: 'done',
			title: 'Done',
			tasks: [
				{
					id: 't4',
					title: 'Project Scaffolding',
					desc: 'Create monorepo structure',
					tags: ['DevOps']
				}
			]
		}
	]);

	let draggedItem = $state<{ id: string; title: string; desc: string; tags: string[] } | null>(
		null
	);
	let draggedFromCol = $state<string | null>(null);

	function handleDragStart(
		event: DragEvent,
		task: { id: string; title: string; desc: string; tags: string[] },
		colId: string
	) {
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
		const item = draggedItem;
		const fromColId = draggedFromCol;
		if (!item || !fromColId) return;
		if (fromColId === targetColId) return;

		// Move item between columns
		columns = columns.map((col) => {
			if (col.id === fromColId) {
				return { ...col, tasks: col.tasks.filter((t) => t.id !== item.id) };
			}
			if (col.id === targetColId) {
				return { ...col, tasks: [...col.tasks, item] };
			}
			return col;
		});

		draggedItem = null;
		draggedFromCol = null;
	}
</script>

<div class="flex h-full w-full scrollbar-thin gap-4 overflow-x-auto p-4">
	{#each columns as col (col.id)}
		<div
			class="flex h-full w-80 shrink-0 flex-col rounded-xl bg-muted/40 p-3"
			ondragover={handleDragOver}
			ondrop={(event) => handleDrop(event, col.id)}
			role="region"
			aria-label="{col.title} column"
		>
			<div class="mb-3 flex items-center justify-between px-1">
				<h3 class="font-semibold">{col.title}</h3>
				<span class="rounded-full bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground"
					>{col.tasks.length}</span
				>
			</div>

			<div class="flex min-h-[150px] flex-1 flex-col gap-2 overflow-y-auto">
				{#each col.tasks as task (task.id)}
					<div
						role="button"
						tabindex="0"
						draggable="true"
						ondragstart={(event) => handleDragStart(event, task, col.id)}
						class="cursor-grab rounded-lg transition-all hover:ring-2 hover:ring-primary/50 active:cursor-grabbing"
					>
						<Card class="shadow-sm">
							<CardHeader class="p-3 pb-0">
								<CardTitle class="text-sm font-medium">{task.title}</CardTitle>
							</CardHeader>
							<CardContent class="p-3 pt-2">
								<p class="mb-3 text-xs text-muted-foreground">{task.desc}</p>
								<div class="mt-auto flex items-center justify-between">
									<div class="flex flex-wrap gap-1">
										{#each task.tags as tag (tag)}
											<Badge variant="secondary" class="h-4 px-1 py-0 text-[10px]">{tag}</Badge>
										{/each}
									</div>
									<div
										class="flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-primary/20 text-[10px] font-bold text-primary"
									>
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
