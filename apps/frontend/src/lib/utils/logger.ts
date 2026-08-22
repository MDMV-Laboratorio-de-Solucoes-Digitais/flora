import { ApiClient } from '../api/client';

export type LogLevel = 'DEBUG' | 'INFO' | 'WARN' | 'ERROR';

export interface LogEntry {
	timestamp: string;
	level: LogLevel;
	module: string;
	message: string;
	context?: Record<string, unknown>;
}

class Logger {
	private buffer: LogEntry[] = [];
	private batchSize = 10;
	private flushIntervalMs = 5000;
	private intervalId: ReturnType<typeof setInterval> | null = null;

	constructor() {
		if (typeof window === 'undefined') {
			return;
		}

		this.intervalId = setInterval(async () => {
			try {
				await this.flush();
			} catch {
				// Silent failure to avoid loops
			}
		}, this.flushIntervalMs);

		window.addEventListener('beforeunload', () => {
			// Use sendBeacon for reliable delivery on unload if possible,
			// or just try to flush normally. Since flush is async, it might not complete.
			if (this.buffer.length === 0) {
				return;
			}

			const entries = [...this.buffer];
			navigator.sendBeacon('/api/v1/telemetry/logs', JSON.stringify(entries));
			this.buffer = [];
		});
	}

	private log(
		level: LogLevel,
		module: string,
		message: string,
		context?: Record<string, unknown>,
		error?: Error
	) {
		const entry: LogEntry = {
			timestamp: new Date().toISOString(),
			level,
			module,
			message: error ? `${message}: ${error.message}` : message,
			context
		};

		this.buffer.push(entry);

		// Also output to console in dev mode, though the rule is no-console.
		// We will disable this rule for this specific file or just not use console at all.
		// Actually the requirement says "console.log is completely forbidden via ESLint".
		// We will not log to console at all, only batch to Rust backend.

		if (this.buffer.length >= this.batchSize) {
			(async () => {
				try {
					await this.flush();
				} catch {
					// Silent
				}
			})();
		}
	}

	public debug(module: string, message: string, context?: Record<string, unknown>): void {
		this.log('DEBUG', module, message, context);
	}

	public info(module: string, message: string, context?: Record<string, unknown>): void {
		this.log('INFO', module, message, context);
	}

	public warn(module: string, message: string, context?: Record<string, unknown>): void {
		this.log('WARN', module, message, context);
	}

	public error(
		module: string,
		message: string,
		context?: Record<string, unknown>,
		error?: Error
	): void {
		this.log('ERROR', module, message, context, error);
	}

	public async flush(): Promise<void> {
		if (this.buffer.length === 0) return;

		const entriesToSend = [...this.buffer];
		this.buffer = []; // Clear buffer immediately

		try {
			await ApiClient.request('/api/v1/telemetry/logs', {
				method: 'POST',
				body: JSON.stringify(entriesToSend)
			});
		} catch {
			// If flush fails, we might want to put them back or just drop them.
			// Dropping them for now to avoid unbounded memory growth if backend is down.
			// A more robust implementation might keep them in local storage.
		}
	}
}

export const logger = new Logger();
