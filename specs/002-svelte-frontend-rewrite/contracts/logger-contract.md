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
    
    // Internal method to batch POST logs to the Rust backend
    flush(): Promise<void>;
}
```

## Backend Ingestion API (Rust Axum)

- **Endpoint**: `POST /api/v1/telemetry/logs`
- **Payload Format**: `Array<LogEntry>`
- **Authentication**: Requires valid OIDC bearer token in headers.
- **Behavior**: The Rust backend parses this array and maps it directly to the `tracing` crate, forwarding to the configured OpenTelemetry Collector.
