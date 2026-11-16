import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

class FakeWebSocket {
	static OPEN = 1;
	static CONNECTING = 0;
	static CLOSED = 3;
	private listeners: Record<string, Array<(event_: unknown) => void>> = {};
	readyState = FakeWebSocket.CONNECTING;
	url: string;
	sent: string[] = [];

	constructor(url: string) {
		this.url = url;
		(sockets as FakeWebSocket[]).push(this);
	}

	addEventListener(type: string, callback: (event_: unknown) => void) {
		this.listeners[type] ??= [];
		this.listeners[type].push(callback);
	}

	send(data: string) {
		this.sent.push(data);
	}

	close(code?: number, reason?: string) {
		this.readyState = FakeWebSocket.CLOSED;
		this.emit('close', { code: code ?? 1000, reason: reason ?? '' });
	}

	emit(type: string, event_: unknown) {
		const callbacks = this.listeners[type] ?? [];
		for (const callback of callbacks) callback(event_);
	}

	open() {
		this.readyState = FakeWebSocket.OPEN;
		this.emit('open', {});
	}
}

let sockets: FakeWebSocket[] = [];

vi.mock('$lib/api/client', () => ({
	API_BASE_URL: 'http://localhost:3000',
	resolveWsBaseUrl: (ws: string | null | undefined, api: string) =>
		typeof ws === 'string' && ws.trim().length > 0 ? ws.trim() : api
}));

vi.mock('$lib/state/MessageStore.svelte', () => ({
	messageStore: { addMessage: vi.fn() }
}));

const authStateMock = {
	token: 'tok-1',
	isAuthenticated: true,
	isSessionLocked: false,
	isInvalidated: false,
	lockSession: vi.fn(),
	invalidate: vi.fn()
};

vi.mock('$lib/state/AuthState.svelte', () => ({
	authState: authStateMock
}));

describe('WsClient', () => {
	beforeEach(() => {
		sockets = [];
		vi.useFakeTimers();
		authStateMock.token = 'tok-1';
		authStateMock.isAuthenticated = true;
		authStateMock.isSessionLocked = false;
		authStateMock.isInvalidated = false;
		authStateMock.lockSession.mockReset();
		authStateMock.invalidate.mockReset();
		vi.stubGlobal('WebSocket', FakeWebSocket as unknown as typeof WebSocket);
	});

	afterEach(() => {
		vi.useRealTimers();
		vi.unstubAllGlobals();
	});

	it('builds /api/v1/ws url with ws scheme and token', async () => {
		const { WsClient, buildWsUrl } = await import('../../src/lib/api/websocket');
		expect(buildWsUrl('http://localhost:3000', 'abc')).toBe(
			'ws://localhost:3000/api/v1/ws?token=abc'
		);
		const client = new WsClient();
		client.connect();
		expect(sockets[0]?.url).toBe('ws://localhost:3000/api/v1/ws?token=tok-1');
	});

	it('parses tagged envelopes safely and ignores unknown/invalid', async () => {
		const { parseWsEnvelope, WsClient } = await import('../../src/lib/api/websocket');
		expect(parseWsEnvelope('not json')).toBeNull();
		expect(parseWsEnvelope(JSON.stringify({ nope: true }))).toBeNull();
		expect(parseWsEnvelope(JSON.stringify({ type: 'Pong', payload: {} }))?.type).toBe('Pong');
		const client = new WsClient();
		expect(() => client.onEnvelope({ type: 'Unknown', payload: { x: 1 } })).not.toThrow();
	});

	it('stops reconnect and locks session on auth/policy close', async () => {
		const { WsClient } = await import('../../src/lib/api/websocket');
		const client = new WsClient();
		client.connect();
		sockets[0].open();
		sockets[0].close(1008, 'Unauthorized');
		await vi.runAllTimersAsync();
		expect(authStateMock.lockSession).toHaveBeenCalledTimes(1);
		expect(sockets).toHaveLength(1);
		expect(client.reconnectTimer).toBeNull();
	});

	it('uses bounded backoff reconnect for retryable close', async () => {
		const { WsClient } = await import('../../src/lib/api/websocket');
		const client = new WsClient();
		client.connect();
		sockets[0].open();
		sockets[0].close(1006, '');
		expect(sockets).toHaveLength(1);
		await vi.advanceTimersByTimeAsync(20_000);
		expect(sockets.length).toBeGreaterThan(1);
	});
});
