<script lang="ts">
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { groupByStatus, isValidMove } from './kanban.svelte';
	import type { BoardTask } from './types';
	import { t } from '$lib/i18n';

	/**
	 * Minimum real Kanban board. Controlled component: the parent owns the task
	 * list (single source of truth) and persists status changes via the API.
	 * Canonical backend status strings: Todo | InProgress | Done | Archived.
	 */
	let {
		tasks,
		onStatusChange
	}: {
		tasks: BoardTask[];
		onStatusChange: (id: string, status: string) => void;
	} = $props();

	// Derived from props — no duplicated local state (avoids drift with the route).
	const grouped = $derived(groupByStatus(tasks));

	let draggedId = $state<string | null>(null);
	let draggedFromStatus = $state<string | null>(null);

	function handleDragStart(event: DragEvent, task: BoardTask, fromStatus: string) {
		draggedId = task.id;
		draggedFromStatus = fromStatus;
		if (event.dataTransfer) {
			event.dataTransfer.effectAllowed = 'move';
			// Firefox requires data to be set for drag to start.
			event.dataTransfer.setData('text/plain', task.id);
		}
	}

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
		if (event.dataTransfer) event.dataTransfer.dropEffect = 'move';
	}

	function handleDrop(event: DragEvent, targetStatus: string) {
		event.preventDefault();
		const id = draggedId;
		const from = draggedFromStatus;
		draggedId = null;
		draggedFromStatus = null;
		if (!id || !isValidMove(from, targetStatus)) return;
		onStatusChange(id, targetStatus);
	}
</script>

<div class="flex h-full w-full scrollbar-thin gap-4 overflow-x-auto p-2">
	{#each grouped as col (col.id)}
		<div
			class="flex h-full min-h-[200px] w-80 shrink-0 flex-col rounded-xl bg-muted/40 p-3"
			ondragover={handleDragOver}
			ondrop={(event) => handleDrop(event, col.status)}
			role="region"
			aria-label={`${t['kanban.column']} ${col.title}`}
		>
			<div class="mb-3 flex items-center justify-between px-1">
				<h3 class="font-semibold">{col.title}</h3>
				<span class="rounded-full bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground"
					>{col.items.length}</span
				>
			</div>

			<div class="flex min-h-[150px] flex-1 flex-col gap-2 overflow-y-auto">
				{#each col.items as task (task.id)}
					<div
						role="button"
						tabindex="0"
						draggable="true"
						ondragstart={(event) => handleDragStart(event, task, col.status)}
						class="cursor-grab rounded-lg transition-all hover:ring-2 hover:ring-primary/50 active:cursor-grabbing"
					>
						<Card class="shadow-sm">
							<CardHeader class="p-3 pb-0">
								<CardTitle class="text-sm font-medium">{task.title}</CardTitle>
							</CardHeader>
							<CardContent class="p-3 pt-2">
								{#if task.description}
									<p class="text-xs text-muted-foreground">{task.description}</p>
								{/if}
							</CardContent>
						</Card>
					</div>
				{/each}
			</div>
		</div>
	{/each}
</div>
