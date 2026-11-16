// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	/** Deploy-time runtime config injected by `/config.js` before the app bundle. */
	interface Window {
		__FLORA_CONFIG__?: {
			apiBaseUrl?: string;
			wsBaseUrl?: string | null;
		};
	}
	// Declared as a global var as well so `globalThis.__FLORA_CONFIG__` type-checks
	// in the Node-based Vitest "server" project, where `window` does not exist.
	var __FLORA_CONFIG__: Window['__FLORA_CONFIG__'];
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
