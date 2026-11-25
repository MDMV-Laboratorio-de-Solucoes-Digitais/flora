export type ThemePreference = 'light' | 'dark';

export const THEME_STORAGE_KEY = 'flora.theme';

export interface ThemeStorageLike {
	getItem(key: string): string | null;
	setItem(key: string, value: string): void;
}

export interface ThemeRootLike {
	classList: {
		contains(token: string): boolean;
		toggle(token: string, shouldForce?: boolean): boolean;
	};
}

export interface ThemeEnvironmentOptions {
	storage?: ThemeStorageLike | null;
	root?: ThemeRootLike | null;
}

function getDefaultStorage(): ThemeStorageLike | null {
	try {
		return globalThis.localStorage ?? null;
	} catch {
		return null;
	}
}

function getDefaultRoot(): ThemeRootLike | null {
	try {
		return globalThis.document?.documentElement ?? null;
	} catch {
		return null;
	}
}

function resolveOptions(options?: ThemeEnvironmentOptions): {
	storage: ThemeStorageLike | null;
	root: ThemeRootLike | null;
} {
	return {
		storage: options?.storage === undefined ? getDefaultStorage() : options.storage,
		root: options?.root === undefined ? getDefaultRoot() : options.root
	};
}

export function parseThemePreference(value: string | null | undefined): ThemePreference | null {
	if (value === 'light' || value === 'dark') {
		return value;
	}
	return null;
}

export function resolveInitialTheme(options?: ThemeEnvironmentOptions): ThemePreference {
	const { storage, root } = resolveOptions(options);
	if (storage) {
		try {
			const stored = parseThemePreference(storage.getItem(THEME_STORAGE_KEY));
			if (stored) {
				return stored;
			}
		} catch {
			// Ignore storage access errors in restricted browser contexts.
		}
	}
	if (root?.classList.contains('dark')) {
		return 'dark';
	}
	return 'light';
}

export function syncThemeClass(
	theme: ThemePreference,
	options?: ThemeEnvironmentOptions
): ThemePreference {
	const { root } = resolveOptions(options);
	root?.classList.toggle('dark', theme === 'dark');
	return theme;
}

export function applyThemePreference(
	theme: ThemePreference,
	options?: ThemeEnvironmentOptions
): ThemePreference {
	const { storage, root } = resolveOptions(options);
	root?.classList.toggle('dark', theme === 'dark');
	if (storage) {
		try {
			storage.setItem(THEME_STORAGE_KEY, theme);
		} catch {
			// Ignore storage write errors in restricted browser contexts.
		}
	}
	return theme;
}
