// Contract for the structured logger utility replacing console.log

export type LogLevel = 'DEBUG' | 'INFO' | 'WARN' | 'ERROR';

export interface LogEntry {
  timestamp: string; // ISO 8601
  level: LogLevel;
  module: string; // e.g., 'AuthService', 'WebSocket'
  message: string;
  context?: Record<string, unknown>;
}

export interface LoggerUtility {
  debug(module: string, message: string, context?: Record<string, unknown>): void;
  info(module: string, message: string, context?: Record<string, unknown>): void;
  warn(module: string, message: string, context?: Record<string, unknown>): void;
  error(module: string, message: string, context?: Record<string, unknown>): void;

  /**
   * Manually flushes the buffer to the Rust backend API.
   * Normally handled automatically via interval.
   */
  flush(): Promise<void>;
}
