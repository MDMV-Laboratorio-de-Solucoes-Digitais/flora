<script lang="ts">
	import { Search } from '@lucide/svelte';
	import { Input } from '$lib/components/ui/input';
	import { ApiClient } from '$lib/api/client';
	import type { SearchResponse, SearchResult } from '$lib/types/models';
	import { normalizeSearchQuery, resultTypeLabel } from '$lib/utils/search';
	import { apiErrorMessage, format, t } from '$lib/i18n';

	let { debounceMs = 250 }: { debounceMs?: number } = $props();

	let query = $state('');
	let results = $state<SearchResult[]>([]);
	let status = $state<'idle' | 'loading' | 'error'>('idle');
	let errorMessage = $state('');

	$effect(() => {
		const q = normalizeSearchQuery(query);
		if (!q) {
			results = [];
			status = 'idle';
			errorMessage = '';
			return;
		}

		let isCancelled = false;
		status = 'loading';
		const timer = setTimeout(() => {
			void (async () => {
				try {
					const response: SearchResponse = await ApiClient.search({ q, limit: 20 });
					if (isCancelled) return;
					results = response.results;
					status = 'idle';
					errorMessage = '';
				} catch (error: unknown) {
					if (isCancelled) return;
					results = [];
					status = 'error';
					errorMessage = apiErrorMessage(error, 'error.searchFailed');
				}
			})();
		}, debounceMs);

		return () => {
			isCancelled = true;
			clearTimeout(timer);
		};
	});
</script>

<div class="relative w-full max-w-md">
	<Search class="absolute top-2.5 left-2.5 h-4 w-4 text-muted-foreground" aria-hidden="true" />
	<Input
		type="search"
		bind:value={query}
		aria-label={t['search.ariaLabel']}
		placeholder={t['search.placeholder']}
		class="w-full bg-muted/50 pl-9 focus-visible:bg-background"
		onkeydown={(event) => {
			if (event.key === 'Escape') query = '';
		}}
	/>

	{#if status !== 'idle' || query}
		<div
			class="absolute top-full left-0 z-30 mt-1 w-full rounded-lg border bg-background shadow-lg"
		>
			{#if status === 'error'}
				<p role="alert" class="px-3 py-2 text-sm text-destructive">{errorMessage}</p>
			{:else}
				<p
					role="status"
					aria-live="polite"
					class="border-b px-3 py-2 text-xs text-muted-foreground"
				>
					{#if status === 'loading'}
						{t['search.searching']}
					{:else if results.length === 0}
						{format('search.noResults', { query: normalizeSearchQuery(query) })}
					{:else}
						{results.length} {t[results.length === 1 ? 'search.result_one' : 'search.result_other']}
					{/if}
				</p>
				{#if results.length > 0}
					<ul class="max-h-80 overflow-auto py-1" aria-label={t['search.resultsLabel']}>
						{#each results as result (result.type + ':' + result.id)}
							<li class="px-3 py-2 text-sm hover:bg-muted/50">
								<span
									class="mr-2 rounded bg-primary/10 px-1.5 py-0.5 text-xs font-medium text-primary"
								>
									{resultTypeLabel(result.type)}
								</span>
								<span class="text-foreground">{result.snippet}</span>
							</li>
						{/each}
					</ul>
				{/if}
			{/if}
		</div>
	{/if}
</div>
