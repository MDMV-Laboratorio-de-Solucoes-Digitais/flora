# Research & Technical Decisions: Flora Seed v0.1 (Svelte 5 Rewrite)

## Decision 1: Svelte 5 Runes for State Management
- **Decision:** Use Svelte 5 Runes (`$state`, `$derived`, `$effect`) exclusively for all reactive state management, encapsulating logic in `.svelte.ts` class files.
- **Rationale:** Runes provide fine-grained reactivity without the lifecycle complexities of Svelte 4 stores. It allows strict TypeScript typing, satisfying our "absolutely zero any types" requirement, and perfectly aligns with the modular monolith architecture.
- **Alternatives considered:** Svelte 4 stores (deprecated in favor of Runes), Redux (too heavy), Pinia (Vue-specific).

## Decision 2: Structured Logger Implementation
- **Decision:** Implement a strictly typed singleton `Logger` class that buffers logs in memory and batch-POSTs them to a dedicated Rust backend endpoint. `console.log` is completely forbidden via ESLint.
- **Rationale:** Directly exposing the OpenTelemetry Collector to the frontend introduces security and CORS risks. The Rust backend already implements `tracing` (OTLP), so proxying frontend logs through Rust ensures a unified, secure telemetry pipeline.
- **Alternatives considered:** Direct browser OTLP exporter (rejected due to security/CORS), overriding `console.log` globally (rejected as it masks poor coding practices).

## Decision 3: WebSocket Synchronization & State Reconciliation
- **Decision:** Implement a "REST Fallback Sync" algorithm upon WebSocket reconnection. The client fetches missed events using `last_known_message_id` via REST, merges and deduplicates them using Svelte 5 Runes, and then resumes processing the real-time stream.
- **Rationale:** Guarantees no message loss or duplication during network blips. It leverages PostgreSQL as the single source of truth rather than relying on complex client-side CRDTs.
- **Alternatives considered:** Replaying missed messages directly through Valkey (complex offset management on the client), Client-side CRDTs (overkill for simple messaging).

## Decision 4: Frontend Dependency Management
- **Decision:** Use `Bun` as the mandatory package manager and script runner.
- **Rationale:** Bun offers significantly faster installation and execution times compared to npm/yarn/pnpm, aligning with our philosophy of lightweight, high-performance tooling.
- **Alternatives considered:** pnpm (slower than Bun), npm (too slow).
