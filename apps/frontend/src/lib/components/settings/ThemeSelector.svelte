<script lang="ts">
	import { onMount } from 'svelte';
	import { Moon, Sun } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { t } from '$lib/i18n';
	import { themeStore } from '$lib/state/ThemeStore.svelte';

	onMount(() => {
		themeStore.syncFromEnvironment();
	});
</script>

<section class="space-y-4 rounded-lg border p-6 shadow-sm" aria-labelledby="theme-title">
	<div class="space-y-2">
		<h2 id="theme-title" class="text-2xl font-bold tracking-tight">{t['theme.title']}</h2>
		<p class="text-sm text-muted-foreground">{t['theme.description']}</p>
	</div>

	<div class="flex flex-wrap items-center gap-3" role="group" aria-label={t['theme.groupLabel']}>
		<Button
			type="button"
			variant={themeStore.theme === 'light' ? 'default' : 'outline'}
			aria-pressed={themeStore.theme === 'light'}
			onclick={() => themeStore.setTheme('light')}
		>
			<Sun class="size-4" aria-hidden="true" />
			<span>{t['theme.light']}</span>
		</Button>
		<Button
			type="button"
			variant={themeStore.theme === 'dark' ? 'default' : 'outline'}
			aria-pressed={themeStore.theme === 'dark'}
			onclick={() => themeStore.setTheme('dark')}
		>
			<Moon class="size-4" aria-hidden="true" />
			<span>{t['theme.dark']}</span>
		</Button>
	</div>

	<p class="text-xs text-muted-foreground" aria-live="polite">
		{themeStore.theme === 'dark' ? t['theme.statusDark'] : t['theme.statusLight']}
	</p>
</section>
