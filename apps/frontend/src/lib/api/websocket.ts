import { authState } from '../state/AuthState.svelte';
import { API_BASE_URL } from './client';
import { messageStore } from '../state/MessageStore.svelte';
import { logger } from '../utils/logger';

export type WsEnvelope =
	| { type: 'Subscribed'; payload?: { channel_id?: string } }
	| { type: 'Unsubscribed'; payload?: { channel_id?: string } }
	| {
			type: 'NewMessage';
			payload?: { id?: string; message?: Record<string, unknown> } & Record<string, unknown>;
	  }
	| { type: 'MessageSent'; payload?: { id?: string } & Record<string, unknown> }
	| { type: 'Pong'; payload?: Record<string, unknown> }
	| { type: 'Error'; payload?: { code?: string; message?: string } & Record<string, unknown> }
	| { type: string; payload?: unknown };

const MAX_RECONNECT_ATTEMPTS = 8;
const BASE_RECONNECT_MS = 1000;
const MAX_RECONNECT_MS = 15_000;

export function buildWsUrl(baseUrl: string, token: string): string {
	const wsUrl = new URL('/api/v1/ws', baseUrl);
	wsUrl.protocol = wsUrl.protocol === 'https:' ? 'wss:' : 'ws:';
	wsUrl.searchParams.set('token', token);
	return wsUrl.href;
}

export function parseWsEnvelope(raw: string): WsEnvelope | null {
	try {
		const data = JSON.parse(raw) as unknown;
		if (
			!data ||
			typeof data !== 'object' ||
			typeof (data as { type?: unknown }).type !== 'string'
		) {
			return null;
		}
		return data as WsEnvelope;
	} catch {
		return null;
	}
}

export function isAuthFailureClose(event: CloseEvent): boolean {
	if ([1008, 4401, 4403].includes(event.code)) return true;
	const reason = (event.reason || '').toLowerCase();
	return ['unauthorized', 'session', 'forbidden', 'auth'].some((token) => reason.includes(token));
}

export function isAuthFailureEnvelope(envelope: WsEnvelope): boolean {
	if (envelope.type !== 'Error') return false;
	const payload = envelope.payload as { code?: string; message?: string } | undefined;
	const code = (payload?.code || '').toLowerCase();
	const message = (payload?.message || '').toLowerCase();
	return (
		code.includes('unauthorized') ||
		code.includes('auth') ||
		code.includes('session') ||
		message.includes('unauthorized') ||
		message.includes('session')
	);
}

export class WsClient {
	private manualClose = false;
	private authFailureHandled = false;

	isConnected = false;
	lastKnownMessageId: string | null = null;
	reconnectTimer: ReturnType<typeof setTimeout> | null = null;
	socket: WebSocket | null = null;
	reconnectAttempts = 0;

	private clearReconnectTimer() {
		if (!this.reconnectTimer) {
			return;
		}

		clearTimeout(this.reconnectTimer);
		this.reconnectTimer = null;
	}

	private handleAuthFailure() {
		this.clearReconnectTimer();
		if (this.authFailureHandled) return;
		this.authFailureHandled = true;
		this.reconnectAttempts = MAX_RECONNECT_ATTEMPTS;
		logger.warn('websocket', 'Authentication/session failure; stopping reconnect');
		if (authState.isAuthenticated) {
			authState.lockSession();
		}
	}

	connect() {
		this.clearReconnectTimer();
		if (
			this.socket &&
			(this.socket.readyState === WebSocket.OPEN || this.socket.readyState === WebSocket.CONNECTING)
		) {
			return;
		}
		if (
			!authState.token ||
			!authState.isAuthenticated ||
			authState.isSessionLocked ||
			authState.isInvalidated
		) {
			return;
		}

		this.manualClose = false;
		this.authFailureHandled = false;
		const url = buildWsUrl(API_BASE_URL, authState.token);
		this.socket = new WebSocket(url);

		this.socket.addEventListener('open', () => {
			this.isConnected = true;
			this.reconnectAttempts = 0;
			this.authFailureHandled = false;
			logger.info('websocket', 'Connected');
		});

		this.socket.addEventListener('message', (event) => {
			if (typeof event.data !== 'string') return;
			const envelope = parseWsEnvelope(event.data);
			if (!envelope) {
				logger.warn('websocket', 'Ignored invalid websocket envelope');
				return;
			}
			this.onEnvelope(envelope);
		});

		this.socket.addEventListener('close', (event) => {
			this.isConnected = false;
			this.socket = null;
			if (this.manualClose) return;
			if (this.authFailureHandled || isAuthFailureClose(event)) {
				this.handleAuthFailure();
				return;
			}
			this.scheduleReconnect();
		});

		this.socket.addEventListener('error', () => {
			// close event determines retry/failure handling
		});
	}

	disconnect() {
		this.manualClose = true;
		this.clearReconnectTimer();
		this.reconnectAttempts = 0;
		if (this.socket) {
			try {
				this.socket.close();
			} catch {
				// ignore
			}
			this.socket = null;
		}
		this.isConnected = false;
	}

	scheduleReconnect() {
		if (
			this.manualClose ||
			this.authFailureHandled ||
			!authState.isAuthenticated ||
			authState.isSessionLocked ||
			authState.isInvalidated
		) {
			return;
		}
		if (this.reconnectAttempts >= MAX_RECONNECT_ATTEMPTS) {
			logger.warn('websocket', 'Max reconnect attempts reached');
			return;
		}
		const attempt = this.reconnectAttempts++;
		const exp = Math.min(MAX_RECONNECT_MS, BASE_RECONNECT_MS * 2 ** attempt);
		const jitter = Math.floor(Math.random() * 250);
		this.reconnectTimer = setTimeout(() => {
			this.reconnectTimer = null;
			if (!this.manualClose && authState.isAuthenticated && !authState.isSessionLocked) {
				this.connect();
			}
		}, exp + jitter);
	}

	// Sync endpoint is not implemented in backend 014; keep safe no-op to avoid retry loops.
	async syncMissedMessages() {
		return;
	}

	onEnvelope(envelope: WsEnvelope) {
		const payload = envelope.payload as Record<string, unknown> | undefined;
		const id =
			(payload?.id as string | undefined) ?? (payload?.message as { id?: string } | undefined)?.id;
		if (id) this.lastKnownMessageId = id;

		switch (envelope.type) {
			case 'Subscribed':
			case 'Unsubscribed':
			case 'Pong':
			case 'MessageSent': {
				return;
			}
			case 'NewMessage': {
				const message = payload?.message as Record<string, unknown> | undefined;
				if (message && typeof message.id === 'string' && typeof message.channel_id === 'string') {
					messageStore.addMessage(message as never);
				}
				return;
			}
			case 'Error': {
				if (isAuthFailureEnvelope(envelope)) {
					this.handleAuthFailure();
				}
				return;
			}
			default: {
				return;
			}
		}
	}

	sendSubscribe(channelId: string) {
		if (!channelId || !this.socket || this.socket.readyState !== WebSocket.OPEN) return false;
		this.socket.send(JSON.stringify({ type: 'Subscribe', payload: { channel_id: channelId } }));
		return true;
	}

	sendUnsubscribe(channelId: string) {
		if (!channelId || !this.socket || this.socket.readyState !== WebSocket.OPEN) return false;
		this.socket.send(JSON.stringify({ type: 'Unsubscribe', payload: { channel_id: channelId } }));
		return true;
	}

	onMessage(event: { type: string; payload?: unknown }) {
		// Back-compat shim for older callers/tests; prefer onEnvelope with tagged envelopes.
		this.onEnvelope(event as WsEnvelope);
	}
}

export const wsClient = new WsClient();
