# Implementation Plan: Flora Seed v0.1 (Svelte 5 Rewrite)

**Branch**: `002-svelte-frontend-rewrite` | **Date**: 2026-08-21 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/002-svelte-frontend-rewrite/spec.md`

## Summary

Execute a strangler fig pattern rewrite of the frontend from SolidJS to Svelte 5 (Runes Mode), using Tailwind CSS and shadcn/UI, enforcing extreme compiler rigor and explicit error handling matching the Rust backend.

## Technical Context

**Language/Version**: TypeScript 5.5, Svelte 5 (Runes Mode)

**Primary Dependencies**: SvelteKit, shadcn-svelte, Tailwind CSS, native fetch wrapper

**Storage**: Local state via Svelte 5 Runes. File blobs to RustFS. Data persistence to PostgreSQL (via Rust backend).

**Testing**: Vitest (Unit), Playwright (E2E)

**Target Platform**: Modern Web Browsers

**Project Type**: Web Application Frontend

**Performance Goals**: Pass the "VPS Test" (lightweight footprint), zero `any` types, zero `svelte-check` warnings.

**Constraints**: ZERO `console.log` in production, extreme linting (clippy::pedantic equivalent).

**Scale/Scope**: Milestone 1 core features (Auth, Channels, Messaging, Tasks, Files).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Debloat & Vendor Unlock**: Passes. Open-source frontend ecosystem (Svelte/Tailwind) without proprietary SaaS lock-in.
- **The VPS Test**: Passes. Frontend compiles to static assets served by the Rust backend or a lightweight proxy.
- **Code Rigor & Linting**: Passes. Enforced via explicit `strict: true`, no `any`, mandatory `svelte-check`, and custom logger.
- **Spec-Driven & Test-First Development**: Passes. Plan designed around testable acceptance criteria.

## Project Structure

### Documentation (this feature)

```text
specs/002-svelte-frontend-rewrite/
├── plan.md              # This file
├── research.md          # Technical decisions (Runes, Logger, Sync)
├── data-model.md        # TS Interfaces
├── quickstart.md        # Validation scenarios
├── contracts/           # Logger interface
└── tasks.md             # (To be generated)
```

### Source Code (repository root)

```text
apps/frontend/
├── src/
│   ├── app.html
│   ├── lib/
│   │   ├── api/            # API client pointing to local Rust backend
│   │   ├── components/     # shadcn/ui and custom components
│   │   ├── state/          # Svelte 5 Runes state (e.g., AuthState.svelte.ts)
│   │   └── utils/          # Logger utility (replaces console.log)
│   └── routes/             # SvelteKit filesystem router
│       ├── (app)/          # Authenticated workspace routes
│       └── (auth)/         # Zitadel OIDC login routes
├── tsconfig.json           # strict: true, noImplicitAny
├── eslint.config.js        # Strict rules (no-console, etc.)
└── package.json
```

**Structure Decision**: A dedicated SvelteKit app in a monorepo structure (`apps/frontend`). Uses `src/lib/state` exclusively for Svelte 5 Rune classes and `src/lib/utils` for the strict logger.

## Architecture Details

### 1. State Management (Svelte 5 Runes)
- Global state managed via classes using `$state` and injected via `setContext/getContext`.
- Example: `MessageStore.svelte.ts` holds `$state(messages[])`.
- **WebSocket Reconnect Logic:** When reconnecting, the UI pauses processing new socket events, calls `GET /api/v1/messages/sync?last_id={lastId}`, merges missing messages into the `$state` array using UUID deduplication, and then resumes the Valkey stream.

### 2. API & Real-time Integration
- API Client acts as a proxy to the local Rust backend running `offline` traits.
- **Session Expiration (401 Interceptor):** A global interceptor wraps all API calls. If a `401 Unauthorized` is returned (e.g., Zitadel token refresh fails), it triggers an event handled by `AuthState.svelte.ts`. This enforces the Hybrid Security Model by locking write operations and placing the session into a secure "Pending Re-authentication / Read-Only Cache" mode for a 5-minute grace period before forcing a clean redirect to login. Additionally, active session invalidation (e.g., user removed from workspace) is driven by Valkey Pub/Sub events which trigger an immediate full-screen UI lock and redirect without a grace period.

### 3. Structured Logging Utility
- A `Logger` class in `src/lib/utils/logger.ts` replaces `console.log`.
- Buffers JSON logs (`{ level, module, message, context }`) locally in memory.
- Uses both a 5-second interval and a `beforeunload` event listener to batch POST to the Rust backend (`/api/v1/telemetry/logs`), which forwards to OpenTelemetry.

### 4. Strict Linting & CI Gates
- `tsconfig.json`: `"strict": true`, `"noImplicitAny": true`, `"strictNullChecks": true`.
- `eslint.config.js`: `"no-console": "error"`, `@typescript-eslint/no-explicit-any: "error"`.
- `svelte-check`: Lefthook CI executes `svelte-check --fail-on-warnings`.

## Milestone Phasing

- **Phase 1: Setup & Linting:** Initialize SvelteKit, configure `tsconfig.json`, strict ESLint, `svelte-check` in CI, and shadcn-svelte with Tailwind.
- **Phase 2: Core Utilities:** Implement the Structured Logger and the Global API Client (with the 401 Session Interceptor).
- **Phase 3: Auth & Identity:** Integrate OIDC (Zitadel) flow and `AuthState` Runes.
- **Phase 4: Real-time Engine:** Implement Valkey WebSocket client, REST fallback sync logic, and `MessageStore` Runes.
- **Phase 5: Task Tracking:** Build Tasks UI, forms, and filtering functionality.
- **Phase 6: File Management:** Build file upload UI, including client-side 50MB pre-flight file size checks and RustFS integration.
