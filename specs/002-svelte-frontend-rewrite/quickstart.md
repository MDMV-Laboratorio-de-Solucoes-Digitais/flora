# Quickstart Validation Guide: Flora Seed v0.1 (Frontend)

This guide provides end-to-end validation scenarios to ensure the Svelte 5 frontend meets the strict architectural constraints of Milestone 1.

## Prerequisites
- **Bun** (v1.1+): Required for frontend dependency management.
- **Docker Compose**: Required to spin up the local "offline" Rust backend, PostgreSQL, and Valkey.

## Setup & Boot
1. **Start Local Backend Infrastructure:**
   ```bash
   # From repository root
   docker compose -f docker-compose.local.yml up -d
   ```
   *Expectation: PostgreSQL, Valkey, and the Rust API (using dev/offline traits) boot successfully.*

2. **Install Frontend Dependencies:**
   ```bash
   cd apps/frontend
   bun install
   ```

3. **Run Code Rigor Validations (Must Pass Before Dev):**
   ```bash
   # 1. Check strict TypeScript & Svelte Runes compilation
   bun run svelte-check --fail-on-warnings
   
   # 2. Check strict ESLint (verifying NO console.log exists)
   bun run lint
   ```
   *Expectation: Both commands execute with exactly ZERO warnings or errors.*

4. **Start Development Server:**
   ```bash
   bun run dev
   ```
   *Expectation: SvelteKit app runs on `http://localhost:5173` and successfully proxies `/api` calls to the local Rust backend.*

## End-to-End Validation Scenarios

### Scenario 1: Authentication & Session Lock
1. Navigate to `http://localhost:5173`.
2. Complete the Zitadel OIDC login flow.
3. Once authenticated on the dashboard, manually stop the local Zitadel container: `docker stop zitadel`.
4. Trigger an action that requires token refresh (or wait for automatic refresh).
5. **Validation:** The frontend MUST intercept the failure, overlay a "Session Expired" UI lock, and enter Read-Only mode. It MUST NOT crash or show an unhandled promise rejection.

### Scenario 2: Real-time Message Sync
1. Open the app in two separate browser windows (User A and User B) side-by-side.
2. In Window A, navigate to a channel and send a message.
3. **Validation:** Window B MUST display the message instantly via Valkey WebSockets.
4. Temporarily disable network throttling in Window B (Offline mode) via DevTools.
5. Send 3 more messages from Window A.
6. Re-enable network in Window B.
7. **Validation:** Window B MUST automatically hit the REST sync endpoint using `last_known_message_id`, retrieve the 3 missed messages, deduplicate them via Svelte 5 Runes, and seamlessly merge them into the UI.

### Scenario 3: Explicit Error Handling on Large Files
1. Attempt to upload a 500MB video file in a channel.
2. **Validation:** The frontend MUST intercept the upload pre-flight, realizing it exceeds the RustFS limit. It MUST display a user-friendly error dialog. Check the browser terminal: there MUST be 0 `console.log` or `console.error` entries; the error MUST be routed internally to the `Logger` utility.
