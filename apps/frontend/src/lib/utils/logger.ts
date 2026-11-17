export type LogLevel = 'DEBUG' | 'INFO' | 'WARN' | 'ERROR';

export interface LogEntry {
	timestamp: string;
	level: LogLevel;
	module: string;
	message: string;
	context?: Record<string, unknown>;
}

// Local-only buffer. There is no telemetry ingestion endpoint in flora-api, so the
// previous transport silently produced 404s on every flush (including normal page load).
const MAX_BUFFERED_ENTRIES = 200;

class Logger {
	private buffer: LogEntry[] = [];

	private log(
		level: LogLevel,
		module: string,
		message: string,
		context?: Record<string, unknown>,
		error?: Error
	): void {
		const entry: LogEntry = {
			timestamp: new Date().toISOString(),
			level,
			module,
			message: error ? `${message}: ${error.message}` : message,
			context
		};

		this.buffer.push(entry);
		if (this.buffer.length > MAX_BUFFERED_ENTRIES) {
			this.buffer.splice(0, this.buffer.length - MAX_BUFFERED_ENTRIES);
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

	/** Defensive copy of the most recent entries for local diagnostics. Never emits network traffic. */
	public snapshot(): readonly LogEntry[] {
		return [...this.buffer];
	}
}

export const logger = new Logger();
