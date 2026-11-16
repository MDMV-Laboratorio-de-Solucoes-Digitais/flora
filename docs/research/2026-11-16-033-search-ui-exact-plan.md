# Search UI slice — exact implementation plan (cycle 033)

**Base:** `dev@763b8c553` (cycle 032 merged, PR #245)
**Date:** 2026-11-16
**Scope:** ONE slice in `apps/frontend` only — make the existing decorative header search input call the
verified `GET /api/v1/search` and render `{type, id, snippet}` results. No backend, schema, or Rust changes.
**Method:** static read of the frontend/backend contracts, tests, and both CI workflows; primary-doc
verification of Svelte 5 runes and Playwright locators. No runtime/DB execution.
**Deliverable status:** research-only. This document is the only file created by this audit; no source file
was modified.

---

## 0. Verdict: **GO** (with 3 guardrails)

The backend contract is verified and stable, the frontend has a single well-contained insertion point, and
the deterministic tests can live in the CI-enforced Vitest `server` project. This is a genuine one-slice,
frontend-only change.

| # | Guardrail | Why it is a condition, not a nice-to-have |
|---|---|---|
| G1 | Deterministic tests **must** be in `tests/unit/*.test.ts` (Vitest `server` project) | CI runs only `--project server` (`.github/workflows/ci.yml:22`, `apps/frontend/vite.config.ts:39-43`). A `src/**/*.svelte.spec.ts` test is real but **not a merge gate** (`:25-33`). |
| G2 | The e2e file **must** be named `*.e2e.ts` | `playwright.config.ts:9` sets `testMatch: '**/*.e2e.ts'`. The existing `tasks.spec.ts` / `files.spec.ts` never run — a known finding (`docs/research/2026-11-15-012-frontend-api-contract-audit.md:32`). |
| G3 | Keep `type` as an open `string` in the UI model + label map with fallback | Rust emits a free-form `String` (`flora-api/src/routes/search.rs:33-41`), not an enum. A narrow TS union would lie about the wire contract. |

**No-go triggers** (any one means stop and re-scope): (a) requirement for a full ARIA combobox with
`aria-activedescendant` + arrow-key result navigation — that is a different, larger a11y slice; (b) requirement
that the browser component test be CI-enforced without editing `.github/workflows/ci.yml`; (c) discovery that
the org context must be sent as `X-Organization-ID` — it is not (see §1.3).

---

## 1. Verified facts (evidence)

### 1.1 The frontend insertion point is exactly one decorative input

`apps/frontend/src/routes/(app)/+layout.svelte:220-227`:

```svelte
<div class="relative w-full max-w-md">
    <Search class="absolute top-2.5 left-2.5 h-4 w-4 text-muted-foreground" />
    <Input
        type="search"
        placeholder="Search messages, tasks, files..."
        class="w-full bg-muted/50 pl-9 focus-visible:bg-background"
    />
</div>
```

No `bind:value`, no `oninput`, no handler, no `aria-label`. `Search` is imported at `:8` and used only at
`:221`. `Input` is imported at `:21` and is also used at `:282` ("Ask AI..."), so the `Input` import must stay.
The `(app)` route group means the header is present on `/channels`, `/dms`, `/tasks`, `/settings`,
`/select-org` — one component drop-in covers all of them.

The wrapper `<div class="relative w-full max-w-md">` is the natural anchor for an absolutely positioned
results panel; the header is `z-10` (`:216-218`) and sidebar `z-20` (`:88-92`), so the panel needs `z-30`.

### 1.2 The backend contract (authoritative, already verified)

`flora-api/src/routes/search.rs`:

- `:19-29` — `SearchQuery { q: String, type: Option<String>, limit: Option<u32> }` (`type` is
  comma-separated).
- `:31-41` — `SearchResult { type: String, id: String, snippet: String }`.
- `:43-53` — `SearchResponse { query, results, source }`. **`source` is present in the wire response even
  though the task brief lists only `{query, results}` — it is additive (`"postgres"`, set at `:82` and `:103`).**
- `:56-58` + `flora-api/src/routes/mod.rs:39` — router mounted at `/api/v1/search` (inner `/` → exact path;
  no trailing slash needed).
- `:60-105` — handler; `:66-69` requires an org; `:77-84` an empty/whitespace `q` returns `200` with
  `results: []` (so the UI should short-circuit and not call the API); `:86` `limit` defaults to 20 and is
  capped at 50; `:87-92` type filter parsing; `:99-104` authoritative Postgres FTS response.
- searchable columns are generated `STORED` tsvectors (`flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql:3-4`,
  `20261118000001_add_tasks_files_search_fts.sql:3-5,13-15`), so seeded rows are searchable without triggers.

### 1.3 No org header is needed; the Bearer token is already injected

`flora-api/src/extractors/auth.rs:218-225` — `require_org()` reads `self.organization_id`, which is populated
from the verified JWT claim (`:187`, `:248`) after server-side session validation (`:58-94`). The frontend
`ApiClient.request` already sets `Authorization: Bearer <token>` from `authState.token`
(`apps/frontend/src/lib/api/client.ts:78-80`); the token lives at
`apps/frontend/src/lib/state/AuthState.svelte.ts:34,143`. **The search method needs no extra headers.**

### 1.4 The existing `ApiClient` is the right extension point

`apps/frontend/src/lib/api/client.ts:73-110` is a single object with one generic `request<T>(endpoint,
options)`. It already: builds `${API_BASE_URL}${endpoint}` (`:75`), handles 401 by locking the session and
throwing typed `ApiError` (`:95-98`), throws typed errors for other non-2xx (`:100-102`), and parses JSON
(`:40-49`). Every other call site uses `ApiClient.request<...>` (e.g.
`routes/(app)/tasks/+page.svelte:38,50,79`, `lib/components/chat/ChatLayout.svelte:42,69`). A typed
`ApiClient.search()` that delegates to `this.request` adds zero new transport behavior.

### 1.5 Where types and tests go

- Models (camelCase UI types): `apps/frontend/src/lib/types/models.ts:1-56`.
- Wire DTOs + mappers (snake_case → UI): `apps/frontend/src/lib/api/mappers.ts:1-44`. **No mapper is needed
  here**: the search wire shape is already the desired UI shape (`query`, `results`, `type`, `id`, `snippet`,
  `source`). This is a deliberate minimal choice (YAGNI).
- Established unit-test pattern: `apps/frontend/tests/unit/mappers.test.ts:1-41`; typed-error/mocked-fetch
  pattern: `apps/frontend/tests/integration/api-interceptor.test.ts:1-95` (note `vi.stubGlobal`,
  `vi.unstubAllGlobals`, `vi.restoreAllMocks`, `authState.logout()` in `afterEach` at `:6-11`).
- Established component-test pattern: `apps/frontend/src/lib/vitest-examples/Welcome.svelte.spec.ts:1-15`
  (`render` from `vitest-browser-svelte`, `page` from `vitest/browser`, `expect.element(...)`).
- Established e2e pattern: `apps/frontend/tests/e2e/tasks.spec.ts:1-37` (role/placeholder/text locators).
- Lint/TS constraints: `eslint.config.js:27-28` → `no-console: error`, `@typescript-eslint/no-explicit-any:
  error`; `tsconfig.json:12-13` → `strict: true`, `noImplicitAny: true`. Use `cause instanceof Error`, never
  `any`, never `console.*`.
- Formatting: tabs, single quotes, semicolon-free (uniform across all cited files); `bun run lint` runs
  `prettier --check . && eslint .` (`package.json:13`).

### 1.6 CI reality

- `.github/workflows/ci.yml:17-24` (required): `bun run check` → `bun run lint` → `bun run test:unit --
  --run --project server` → `bun run build`. There is **no e2e step** in `ci.yml`.
- `.github/workflows/rust.yml:112-116,162-173` — one Chromium smoke, `continue-on-error: true`
  (informational); `bun run test:e2e` at `:167`; `retries: 0`/`workers: 1` because the injected session jti
  is single-use (`:112-113`, mirrored in `playwright.config.ts:11-15`).
- `apps/frontend/vite.config.ts:20` — `expect: { requireAssertions: true }`: every test needs ≥1 assertion.
- `apps/frontend/playwright.config.ts:18-25` — setup project mints `playwright/.auth/alice.json`
  (`:5`, generated by `tests/e2e/seed.setup.ts:12-72`), consumed by the `chromium` project via
  `storageState`.

### 1.7 Primary-doc facts used (verified this session)

| Source | Fact applied |
|---|---|
| <https://svelte.dev/docs/svelte/$effect> | "can be used for … making network requests"; an effect **can return a teardown function** that runs immediately before re-runs and on component destruction; dependencies are values read **synchronously** in the effect body (reads after `await`/inside `setTimeout` are not tracked); effects should not be used to compute derived state — use `$derived`. |
| <https://svelte.dev/docs/svelte/bind> | Component-level `bind:value` requires the child to declare `value = $bindable()` — confirmed locally at `apps/frontend/src/lib/components/ui/input/input.svelte:13-20`. |
| <https://svelte.dev/docs/svelte/$state> | `$state` for local reactive values (used throughout the layout at `:28-31`). |
| <https://playwright.dev/docs/locators> | Prefer `getByRole()` with accessible name, `getByPlaceholder()`, `getByText()` for non-interactive elements; locators auto-retry. |
| <https://playwright.dev/docs/test-assertions> | `expect(locator).toBeVisible()` auto-waits — absorbs the 250 ms debounce without a manual sleep. |
| <https://playwright.dev/docs/mock> | `page.route(url, route => route.fulfill(...))` — used for a DB-independent deterministic e2e. |

---

## 2. Design decisions (locked)

1. **Debounce via `$effect` + teardown.** Read `query` synchronously into `q` so it is tracked; `setTimeout`
   the request; return a teardown that sets `cancelled = true` and `clearTimeout`. This directly uses the
   documented teardown semantics and prevents stale responses from overwriting newer ones. We do *not* use
   `$effect` to derive a value — we use it for a network side effect, which the docs explicitly sanction.
2. **Typed method, not raw call sites.** `ApiClient.search({q, type?, limit?}): Promise<SearchResponse>` keeps
   URL encoding in one place and makes the pure unit test trivial.
3. **No mapper module.** Wire shape == UI shape; adding `toSearchResult()` would be ceremony.
4. **One pure helper module** (`lib/utils/search.ts`) for query normalization and entity labels, so the
   "entity type + snippet" behavior has a deterministic, browser-free unit test while still being reused by
   the component (DRY).
5. **A11y floor for this slice:** accessible name on the searchbox (`role=searchbox` via `type="search"`),
   a single `role="status" aria-live="polite"` line for searching/empty/count, `role="alert"` for errors,
   `aria-hidden="true"` on the decorative icon, `aria-label` on the results `<ul>`, and Escape-to-clear.
   Full combobox semantics and result keyboard navigation are explicitly out of scope (see G3/no-go).
6. **Snippets render as plain text.** Svelte escapes `{result.snippet}` automatically; do **not** use
   `{@html}`. The server already truncates to 200 chars (`search.rs:145,196-197,248-249`), so no re-truncate.
7. **Empty query short-circuits client-side** (`normalizeSearchQuery(q) === ''` → clear results/status, no
   request), mirroring `search.rs:77-84`.

---

## 3. Exact code to write

### 3.1 `apps/frontend/src/lib/types/models.ts` — append after line 56

```ts
/** Entity kind emitted by GET /api/v1/search (flora-api/src/routes/search.rs:31-41). */
export type SearchResultType = string;

export interface SearchResult {
	/** "message" | "task" | "file" today; open string to tolerate backend additions. */
	type: SearchResultType;
	id: UUID;
	snippet: string;
}

export interface SearchResponse {
	query: string;
	results: SearchResult[];
	/** Authoritative backend; currently always "postgres" (search.rs:52, :103). */
	source: string;
}
```

### 3.2 `apps/frontend/src/lib/api/client.ts` — add one import and one method

At `:1`, add below the existing import:

```ts
import type { SearchResponse } from '../types/models';
```

Inside the `ApiClient` object (`:73-110`), after `request`:

```ts
	/** GET /api/v1/search?q=… — authoritative Postgres FTS (search.rs:60-105). */
	async search(params: { q: string; type?: string; limit?: number }): Promise<SearchResponse> {
		const query = new URLSearchParams({ q: params.q });
		if (params.type) query.set('type', params.type);
		if (params.limit !== undefined) query.set('limit', String(params.limit));
		return this.request<SearchResponse>(`/api/v1/search?${query.toString()}`);
	}
```

`URLSearchParams` encodes a space as `+` and escapes user input, so no manual `encodeURIComponent` is needed.
No header code: `:78-80` already injects the Bearer token.

### 3.3 `apps/frontend/src/lib/utils/search.ts` — NEW file

```ts
const RESULT_LABELS: Readonly<Record<string, string>> = {
	message: 'Message',
	task: 'Task',
	file: 'File'
};

/** Stable display label for an entity type; unknown kinds fall back to the raw token. */
export function resultTypeLabel(type: string): string {
	return RESULT_LABELS[type] ?? type;
}

/** Trimmed query. Empty means "do not call the API" (search.rs:77-84). */
export function normalizeSearchQuery(value: string): string {
	return value.trim();
}
```

### 3.4 `apps/frontend/src/lib/components/search/SearchBox.svelte` — NEW file

```svelte
<script lang="ts">
	import { Search } from 'lucide-svelte';
	import { Input } from '$lib/components/ui/input';
	import { ApiClient } from '$lib/api/client';
	import type { SearchResponse, SearchResult } from '$lib/types/models';
	import { normalizeSearchQuery, resultTypeLabel } from '$lib/utils/search';

	let { debounceMs = 250 }: { debounceMs?: number } = $props();

	let query = $state('');
	let results = $state<SearchResult[]>([]);
	let status = $state<'idle' | 'loading' | 'error'>('idle');
	let errorMessage = $state('');

	$effect(() => {
		// Synchronous read => tracked dependency (svelte.dev/docs/svelte/$effect).
		const q = normalizeSearchQuery(query);
		if (!q) {
			results = [];
			status = 'idle';
			errorMessage = '';
			return;
		}

		let cancelled = false;
		status = 'loading';
		const timer = setTimeout(() => {
			void (async () => {
				try {
					const response: SearchResponse = await ApiClient.search({ q, limit: 20 });
					if (cancelled) return;
					results = response.results;
					status = 'idle';
					errorMessage = '';
				} catch (cause: unknown) {
					if (cancelled) return;
					results = [];
					status = 'error';
					errorMessage = cause instanceof Error ? cause.message : 'Search failed.';
				}
			})();
		}, debounceMs);

		// Runs before every re-run and on unmount (svelte.dev/docs/svelte/$effect).
		return () => {
			cancelled = true;
			clearTimeout(timer);
		};
	});
</script>

<div class="relative w-full max-w-md">
	<Search
		class="absolute top-2.5 left-2.5 h-4 w-4 text-muted-foreground"
		aria-hidden="true"
	/>
	<Input
		type="search"
		bind:value={query}
		aria-label="Search messages, tasks, and files"
		placeholder="Search messages, tasks, files..."
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
						Searching…
					{:else if results.length === 0}
						No results for “{normalizeSearchQuery(query)}”.
					{:else}
						{results.length} {results.length === 1 ? 'result' : 'results'}
					{/if}
				</p>
				{#if results.length > 0}
					<ul class="max-h-80 overflow-auto py-1" aria-label="Search results">
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
```

`debounceMs` is a testability seam (default 250 keeps production behavior unchanged). The `Input` component
forwards `aria-label`/`onkeydown` through `{...restProps}` and supports `bind:value` because `value` is
`$bindable()` (`apps/frontend/src/lib/components/ui/input/input.svelte:13-20,37-47`). Don't add a wrapper
`<label>`: the visible placeholder is not an accessible name, hence the explicit `aria-label`.

### 3.5 `apps/frontend/src/routes/(app)/+layout.svelte` — targeted edit

- At `:8`, remove `Search,` from the `lucide-svelte` import list (it moves into `SearchBox.svelte`).
- Add `import SearchBox from '$lib/components/search/SearchBox.svelte';` next to the other `$lib` imports.
- Replace `:220-227` with:

```svelte
			<div class="flex flex-1 items-center gap-4">
				<SearchBox />
			</div>
```

Leave the `Input` import (`:21`) and the "Ask AI..." input (`:282`) untouched.

### 3.6 `apps/frontend/tests/unit/searchApi.test.ts` — NEW file (CI-enforced, deterministic)

```ts
import { afterEach, describe, expect, it, vi } from 'vitest';
import { API_BASE_URL, ApiClient } from '../../src/lib/api/client';
import { authState } from '../../src/lib/state/AuthState.svelte';
import { normalizeSearchQuery, resultTypeLabel } from '../../src/lib/utils/search';

describe('ApiClient.search', () => {
	afterEach(() => {
		vi.unstubAllGlobals();
		authState.logout();
	});

	it('builds the encoded query and returns the typed SearchResponse', async () => {
		const body = {
			query: 'dashboard',
			results: [
				{
					type: 'task',
					id: '44444444-5555-6666-7777-888888888888',
					snippet: 'Design new dashboard UI Create Figma mockups for the analytics dashboard redesign'
				}
			],
			source: 'postgres'
		};
		const fetchMock = vi.fn().mockResolvedValue({
			ok: true,
			status: 200,
			statusText: 'OK',
			text: () => Promise.resolve(JSON.stringify(body))
		});
		vi.stubGlobal('fetch', fetchMock);
		authState.login('test-token', {});

		const response = await ApiClient.search({ q: 'dashboard words', type: 'tasks', limit: 5 });

		expect(fetchMock).toHaveBeenCalledTimes(1);
		const [url, init] = fetchMock.mock.calls[0] as [string, RequestInit];
		expect(url).toBe(`${API_BASE_URL}/api/v1/search?q=dashboard+words&type=tasks&limit=5`);
		expect((init.headers as Headers).get('Authorization')).toBe('Bearer test-token');
		expect(response).toEqual(body);
	});
});

describe('search helpers', () => {
	it('normalizes whitespace-only queries to empty', () => {
		expect(normalizeSearchQuery('   ')).toBe('');
		expect(normalizeSearchQuery('  dashboard  ')).toBe('dashboard');
	});

	it('maps known entity types and falls back for unknown ones', () => {
		expect(resultTypeLabel('message')).toBe('Message');
		expect(resultTypeLabel('task')).toBe('Task');
		expect(resultTypeLabel('file')).toBe('File');
		expect(resultTypeLabel('poll')).toBe('poll');
	});
});
```

The fixture uses the seeded task title/description verbatim (`dev-seed.sql:283-284`) so the test and the
optional real-API e2e share one source of truth for expected text.

### 3.7 `apps/frontend/tests/e2e/search.e2e.ts` — NEW file (optional, DB-independent)

```ts
import { expect, test } from '@playwright/test';

const SEARCH_ROUTE = /\/api\/v1\/search(?:\?|$)/;

test.describe('Search UI', () => {
	test('renders entity type and snippet', async ({ page }) => {
		await page.route(SEARCH_ROUTE, (route) =>
			route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify({
					query: 'dashboard',
					results: [
						{
							type: 'task',
							id: '44444444-5555-6666-7777-888888888888',
							snippet: 'Design new dashboard UI Create Figma mockups'
						}
					],
					source: 'postgres'
				})
			})
		);

		await page.goto('/tasks');
		await page
			.getByRole('searchbox', { name: 'Search messages, tasks, and files' })
			.fill('dashboard');

		await expect(page.getByText('Task', { exact: true })).toBeVisible();
		await expect(page.getByText('Design new dashboard UI Create Figma mockups')).toBeVisible();
		await expect(page.getByText('1 result')).toBeVisible();
	});

	test('shows the empty state', async ({ page }) => {
		await page.route(SEARCH_ROUTE, (route) =>
			route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify({ query: 'nothingmatches', results: [], source: 'postgres' })
			})
		);

		await page.goto('/tasks');
		await page
			.getByRole('searchbox', { name: 'Search messages, tasks, and files' })
			.fill('nothingmatches');

		await expect(page.getByText('No results for', { exact: false })).toBeVisible();
	});
});
```

`page.route` makes this deterministic without DB/FTS drift. The `chromium` project still needs the auth
`storageState` from the setup project (`playwright.config.ts:18-25`), which is already automatic.
Alternative real-API variant: drop the `page.route` block and search `'dashboard'` — the seeded task at
`dev-seed.sql:283-284` is indexed by the generated tsvector (§1.2); the assertion becomes
`expect(page.getByRole('listitem').filter({ hasText: 'dashboard' })).toHaveCount(1)` per
<https://playwright.dev/docs/locators>. Prefer the mocked variant as the gate; keep the real one as a manual
smoke only.

### 3.8 `apps/frontend/src/lib/components/search/SearchBox.svelte.spec.ts` — NEW file (optional, NOT CI-enforced)

```ts
import { page } from 'vitest/browser';
import { describe, expect, it, vi } from 'vitest';
import { render } from 'vitest-browser-svelte';
import { ApiClient } from '$lib/api/client';
import SearchBox from './SearchBox.svelte';

describe('SearchBox.svelte', () => {
	it('renders entity label and snippet after a debounced search', async () => {
		const search = vi.spyOn(ApiClient, 'search').mockResolvedValue({
			query: 'dashboard',
			results: [{ type: 'task', id: 'id-1', snippet: 'Design new dashboard UI' }],
			source: 'postgres'
		});
		render(SearchBox, { debounceMs: 0 });

		await page.getByRole('searchbox').fill('dashboard');

		await expect.element(page.getByText('Design new dashboard UI')).toBeInTheDocument();
		await expect.element(page.getByText('Task', { exact: true })).toBeInTheDocument();
		expect(search).toHaveBeenCalledWith({ q: 'dashboard', limit: 20 });
		search.mockRestore();
	});
});
```

Pattern follows `src/lib/vitest-examples/Welcome.svelte.spec.ts:1-15`. Avoid fake timers here: Vitest
browser-mode's `expect.element` polling needs real timers; `debounceMs: 0` is the deterministic seam.

---

## 4. Ordered edit list (minimal; 8 files, 3 modified / 5 new)

| # | File | Action | Depends on |
|---|---|---|---|
| 1 | `apps/frontend/src/lib/types/models.ts` | Append `SearchResultType`, `SearchResult`, `SearchResponse` (§3.1) | — |
| 2 | `apps/frontend/src/lib/api/client.ts` | Add type import + `ApiClient.search()` (§3.2) | 1 |
| 3 | `apps/frontend/src/lib/utils/search.ts` | NEW — pure `resultTypeLabel` + `normalizeSearchQuery` (§3.3) | — |
| 4 | `apps/frontend/tests/unit/searchApi.test.ts` | NEW — deterministic server-project tests (§3.6) | 1,2,3 |
| 5 | `bun run test:unit -- --run --project server` | **Verify red→green after 1-4** (tests pass end) | 4 |
| 6 | `apps/frontend/src/lib/components/search/SearchBox.svelte` | NEW — UI + debounce + states + a11y (§3.4) | 1,2,3 |
| 7 | `apps/frontend/src/routes/(app)/+layout.svelte` | Swap decorative input for `<SearchBox />`; drop `Search` import (§3.5) | 6 |
| 8 | `apps/frontend/tests/e2e/search.e2e.ts` | NEW — optional deterministic e2e (§3.7) | 7 |
| 9 | `apps/frontend/src/lib/components/search/SearchBox.svelte.spec.ts` | NEW — optional client-project test (§3.8) | 6 |

TDD note: step 4's tests are written before step 2's implementation within the same commit-sized slice — write
the test, run it red, add the method, run it green. Steps 1-5 and 6-9 are two independently reviewable slices;
commit them separately (`feat(frontend): typed search API client` then `feat(frontend): wire header search UI`).

---

## 5. Verification commands (from `apps/frontend`)

```bash
bun install                                      # if node_modules absent
bun run check                                    # svelte-check — CI gate (ci.yml:18)
bun run lint                                     # prettier --check + eslint — CI gate (ci.yml:20)
bun run test:unit -- --run --project server      # CI gate (ci.yml:22) — MUST pass
bun run build                                    # CI gate (ci.yml:24)
bun run test:unit -- --run --project client      # optional local (SearchBox.svelte.spec.ts)
npx playwright test tests/e2e/search.e2e.ts      # optional local; needs API+DB+auth setup
```

**Expected red→green for step 5:** before §3.2, `searchApi.test.ts` fails with
`Property 'search' does not exist on type '{ request<T>(...) }'`; after §3.2 it passes with 3 assertions
groups. Full green: `svelte-check` reports 0 errors, `prettier`/`eslint` clean, server project all passing,
build succeeds. No Rust command needed — this slice touches no Rust file (the informational `rust.yml` e2e
job, if it runs, exercises the new `search.e2e.ts` automatically because `testMatch` matches).

---

## 6. Risks / unknowns (NVs)

| ID | Risk / unknown | Mitigation / decision |
|---|---|---|
| NV-1 | `source` field exists but the task brief omitted it | Include it in `SearchResponse` (§3.1); tolerant cast means an extra field would be harmless either way. |
| NV-2 | Backend `type` is free-form `String` (`search.rs:36`) | Open `string` + label fallback (G3); no narrowing. |
| NV-3 | Out-of-order responses while typing | `cancelled` flag + teardown in `$effect` (§3.4) — documented Svelte teardown semantics. |
| NV-4 | Writing `status`/`results` inside `$effect` | Docs discourage effects for *derived* state, not for network side effects (§1.7). The effect never reads those values, so no loop. Alternative if reviewers object: `oninput` handler + timer ref + `onMount` cleanup. |
| NV-5 | E2E files silently not discovered | Name `search.e2e.ts` (G2; `playwright.config.ts:9`). |
| NV-6 | Component test invisible to CI | Keep deterministic assertions in `tests/unit/` (G1); component test is local-only unless `ci.yml:22` is changed. |
| NV-7 | `no-console` / `no-explicit-any` are errors | `cause instanceof Error` + typed DTOs; no `console`, no `any`. |
| NV-8 | A 401 during search triggers `authState.lockSession()` (`client.ts:95-98`) | The UI will show the error message while the existing session-lock flow runs; acceptable for this slice, no special handling. |
| NV-9 | Results panel overlaps main content | Absolute panel with `z-30` inside the `relative` header container (§1.1, §3.4). |
| NV-10 | Escaped snippets | Plain-text interpolation only; never `{@html}`. Server truncates at 200 chars. |
| NV-11 | UI copy language | Repo UI strings are English (`+layout.svelte:129,155,277-279`); keep English, including curly-quote empty state. |
| NV-12 | Real-API e2e depends on FTS/seed | Mocked `page.route` variant is the deterministic default; real-API variant is a manual smoke with `dev-seed.sql:283-284` + generated tsvector. |
| NV-13 | `debounceMs` prop is a test seam | Production default 250 ms; no behavior change. If reviewers reject the prop, use `vi.useFakeTimers()` only in the client project and re-test. |

**Not resolved by this slice:** result click-through/navigation to the entity, entity icons beyond the text
label, highlighting matched terms, type filter controls, pagination above the backend's 50-item cap. These
are separate slices, not silent omissions of this one.
