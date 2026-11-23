import { authState } from '../state/AuthState.svelte';
import { API_BASE_URL, resolveWsBaseUrl } from './client';
import { ApiClient } from './client';
import { messageStore } from '../state/MessageStore.svelte';
import { toMessage, type ApiMessage } from './mappers';
import { logger } from '../utils/logger';

export interface WsMessagePayload {
	id: string;
	channel_id: string;
	sender_id: string;
	sender_name?: string | null;
	sender_avatar_url?: string | null;
	content: string;
	thread_id?: string | null;
	is_edited?: boolean;
	created_at: string;
	updated_at?: string | null;
}

export type WsEnvelope =
	| { type: 'Subscribed'; payload?: { channel_id?: string } }
	| { type: 'Unsubscribed'; payload?: { channel_id?: string } }
	| { type: 'NewMessage'; payload?: { channel_id?: string; message?: WsMessagePayload } }
	| { type: 'MessageUpdated'; payload?: { channel_id?: string; message?: WsMessagePayload } }
	| { type: 'MessageDeleted'; payload?: { channel_id?: string; message?: WsMessagePayload } }
	| { type: 'MessageSent'; payload?: { message_id?: string; channel_id?: string } }
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
	private activeChannelId: string | null = null;

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
		const wsBase = resolveWsBaseUrl(globalThis.__FLORA_CONFIG__?.wsBaseUrl, API_BASE_URL);
		const url = buildWsUrl(wsBase, authState.token);
		this.socket = new WebSocket(url);

		this.socket.addEventListener('open', () => {
			this.isConnected = true;
			this.reconnectAttempts = 0;
			this.authFailureHandled = false;
			logger.info('websocket', 'Connected');
			if (this.activeChannelId) {
				this.sendSubscribe(this.activeChannelId);
				void this.syncMissedMessages(this.activeChannelId);
			}
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

	setActiveChannel(channelId: string | null = null) {
		if (this.activeChannelId && this.activeChannelId !== channelId) {
			this.sendUnsubscribe(this.activeChannelId);
		}
		this.activeChannelId = channelId;
		if (!channelId) return;
		this.sendSubscribe(channelId);
		void this.syncMissedMessages(channelId);
	}

	async syncMissedMessages(channelId: string) {
		if (!this.lastKnownMessageId) return;
		try {
			const missed = await ApiClient.request<ApiMessage[]>(
				`/api/v1/channels/${channelId}/messages?after=${encodeURIComponent(this.lastKnownMessageId)}&limit=100`
			);
			for (const message of missed) {
				const mapped = toMessage(message);
				if (mapped.channelId === channelId) {
					messageStore.upsertMessage(mapped);
					this.lastKnownMessageId = mapped.id;
				}
			}
		} catch (error) {
			const detail = error instanceof Error ? error.message : String(error);
			logger.warn('websocket', 'Catch-up sync failed', { detail });
		}
	}

	onEnvelope(envelope: WsEnvelope) {
		const payload = envelope.payload as Record<string, unknown> | undefined;

		switch (envelope.type) {
			case 'Subscribed':
			case 'Unsubscribed':
			case 'Pong':
			case 'MessageSent': {
				const id = payload?.message_id as string | undefined;
				if (id) this.lastKnownMessageId = id;
				return;
			}
			case 'NewMessage':
			case 'MessageUpdated': {
				const message = payload?.message as WsMessagePayload | undefined;
				if (message && typeof message.id === 'string') {
					const mapped = toMessage(message as unknown as ApiMessage);
					messageStore.upsertMessage(mapped);
					this.lastKnownMessageId = mapped.id;
				}
				return;
			}
			case 'MessageDeleted': {
				const message = payload?.message as WsMessagePayload | undefined;
				if (message && typeof message.id === 'string') {
					messageStore.removeMessage(message.id);
					this.lastKnownMessageId = message.id;
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
