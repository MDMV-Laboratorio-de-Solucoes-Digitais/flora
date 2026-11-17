# Logger Interface Contract

This contract defines the strict structured logging utility that entirely replaces `console.log` in the Svelte 5 frontend.

## TypeScript Interface

```typescript
export type LogLevel = 'DEBUG' | 'INFO' | 'WARN' | 'ERROR';

export interface LogEntry {
    timestamp: string; // ISO 8601
    level: LogLevel;
    module: string;
    message: string;
    context?: Record<string, unknown>; // Must be JSON serializable
}

export interface LoggerUtility {
    debug(module: string, message: string, context?: Record<string, unknown>): void;
    info(module: string, message: string, context?: Record<string, unknown>): void;
    warn(module: string, message: string, context?: Record<string, unknown>): void;
    error(module: string, message: string, context?: Record<string, unknown>, error?: Error): void;

    /** Defensive copy of the bounded in-memory buffer for local diagnostics. */
    snapshot(): readonly LogEntry[];
}
```

## Local behavior (cycle 049)

- **No network**: the logger never calls `fetch`, `sendBeacon` or `ApiClient`. There is
  no `POST /api/v1/telemetry/logs` endpoint in `flora-api`; the previous transport
  produced silent 404s on every flush, including normal page load.
- **Buffer**: at most 200 most-recent `LogEntry` values are kept in memory; older
  entries are discarded to avoid unbounded growth.
- **No console output**: the `no-console` rule remains in force; diagnostics use
  `snapshot()` instead of `console.*`.
- **Backend ingestion**: removed from scope. There is no OTel collector or exporter
  in this checkout; reintroducing ingestion requires a separate decision and a real
  authenticated endpoint.
