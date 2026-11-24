import {
	applyThemePreference,
	resolveInitialTheme,
	syncThemeClass,
	type ThemeEnvironmentOptions,
	type ThemePreference
} from '$lib/utils/theme';

export class ThemeStore {
	theme = $state<ThemePreference>('light');

	syncFromEnvironment(options?: ThemeEnvironmentOptions): ThemePreference {
		const resolved = resolveInitialTheme(options);
		this.theme = resolved;
		return syncThemeClass(resolved, options);
	}

	setTheme(next: ThemePreference, options?: ThemeEnvironmentOptions): ThemePreference {
		const applied = applyThemePreference(next, options);
		this.theme = applied;
		return applied;
	}
}

export const themeStore = new ThemeStore();
