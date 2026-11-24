import { describe, expect, it } from 'vitest';
import { ThemeStore } from '$lib/state/ThemeStore.svelte';
import {
	THEME_STORAGE_KEY,
	applyThemePreference,
	parseThemePreference,
	resolveInitialTheme,
	type ThemeRootLike,
	type ThemeStorageLike
} from '$lib/utils/theme';

function createMemoryStorage(initial: Record<string, string> = {}): ThemeStorageLike & {
	data: Map<string, string>;
} {
	const data = new Map<string, string>(Object.entries(initial));
	return {
		data,
		getItem(key: string) {
			return data.get(key) ?? null;
		},
		setItem(key: string, value: string) {
			data.set(key, value);
		}
	};
}

function createFakeRoot(isInitiallyDark = false): ThemeRootLike & { isDark: () => boolean } {
	const tokens = new Set<string>(isInitiallyDark ? ['dark'] : []);
	return {
		isDark: () => tokens.has('dark'),
		classList: {
			contains(token: string) {
				return tokens.has(token);
			},
			toggle(token: string, shouldForce?: boolean) {
				const shouldAdd = shouldForce ?? !tokens.has(token);
				if (shouldAdd) {
					tokens.add(token);
					return true;
				}
				tokens.delete(token);
				return false;
			}
		}
	};
}

describe('theme utilities', () => {
	it('parses only explicit light and dark values', () => {
		expect(parseThemePreference('light')).toBe('light');
		expect(parseThemePreference('dark')).toBe('dark');
		expect(parseThemePreference('system')).toBeNull();
		expect(parseThemePreference('')).toBeNull();
		expect(parseThemePreference(null)).toBeNull();
		expect(parseThemePreference(undefined)).toBeNull();
	});

	it('resolves initial theme from storage first, falling back to root class or light', () => {
		expect(
			resolveInitialTheme({
				storage: createMemoryStorage({ [THEME_STORAGE_KEY]: 'dark' }),
				root: createFakeRoot(false)
			})
		).toBe('dark');

		expect(
			resolveInitialTheme({
				storage: createMemoryStorage({ [THEME_STORAGE_KEY]: 'light' }),
				root: createFakeRoot(true)
			})
		).toBe('light');

		expect(
			resolveInitialTheme({
				storage: createMemoryStorage({ [THEME_STORAGE_KEY]: 'unexpected' }),
				root: createFakeRoot(true)
			})
		).toBe('dark');

		expect(
			resolveInitialTheme({
				storage: {
					getItem() {
						throw new Error('storage blocked');
					},
					setItem() {}
				},
				root: createFakeRoot(true)
			})
		).toBe('dark');

		expect(resolveInitialTheme({ storage: null, root: null })).toBe('light');
	});

	it('applies dark class and persists preference without failing when storage throws', () => {
		const storage = createMemoryStorage();
		const root = createFakeRoot(false);

		expect(applyThemePreference('dark', { storage, root })).toBe('dark');
		expect(root.isDark()).toBe(true);
		expect(storage.data.get(THEME_STORAGE_KEY)).toBe('dark');

		expect(applyThemePreference('light', { storage, root })).toBe('light');
		expect(root.isDark()).toBe(false);
		expect(storage.data.get(THEME_STORAGE_KEY)).toBe('light');

		const brokenStorage: ThemeStorageLike = {
			getItem: () => null,
			setItem() {
				throw new Error('quota exceeded');
			}
		};
		expect(applyThemePreference('dark', { storage: brokenStorage, root })).toBe('dark');
		expect(root.isDark()).toBe(true);
	});
});

describe('ThemeStore', () => {
	it('synchronizes initial state from environment and switches themes', () => {
		const storage = createMemoryStorage({ [THEME_STORAGE_KEY]: 'dark' });
		const root = createFakeRoot(false);
		const store = new ThemeStore();

		expect(store.theme).toBe('light');
		expect(store.syncFromEnvironment({ storage, root })).toBe('dark');
		expect(store.theme).toBe('dark');
		expect(root.isDark()).toBe(true);

		expect(store.setTheme('light', { storage, root })).toBe('light');
		expect(store.theme).toBe('light');
		expect(root.isDark()).toBe(false);
		expect(storage.data.get(THEME_STORAGE_KEY)).toBe('light');
	});
});
