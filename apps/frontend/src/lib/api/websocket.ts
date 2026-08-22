import { authState } from '../state/AuthState.svelte';
import { ApiClient, API_BASE_URL } from './client';
import { messageStore } from '../state/MessageStore.svelte';
import type { Message } from '../types/models';
import { logger } from '../utils/logger';

export class WsClient {
	isConnected = false;
	lastKnownMessageId: string | null = null;
	reconnectTimer: ReturnType<typeof setTimeout> | null = null;
	socket: WebSocket | null = null;

	connect() {
		if (this.socket) {
			this.socket.close();
		}

		if (!authState.token) {
			logger.warn('websocket', 'Cannot connect without an auth token');
			return;
		}

		const wsUrl = new URL('/api/v1/ws', API_BASE_URL);
		wsUrl.protocol = wsUrl.protocol === 'https:' ? 'wss:' : 'ws:';
		wsUrl.searchParams.set('token', authState.token);

		this.socket = new WebSocket(wsUrl.href);

		this.socket.addEventListener('open', () => {
			this.isConnected = true;
			if (this.reconnectTimer) clearTimeout(this.reconnectTimer);
			logger.info('websocket', 'Connected to Valkey pub/sub endpoint');
		});

		this.socket.addEventListener('message', (event) => {
			try {
				const data = JSON.parse(event.data);
				this.onMessage(data);
			} catch (error) {
				logger.error('websocket', 'Failed to parse message', undefined, error as Error);
			}
		});

		this.socket.addEventListener('close', () => {
			this.isConnected = false;
			this.socket = null;
			this.scheduleReconnect();
		});

		this.socket.addEventListener('error', () => {
			logger.error('websocket', 'WebSocket error');
		});
	}

	disconnect() {
		if (this.socket) {
			this.socket.close();
			this.socket = null;
		}
		this.isConnected = false;
	}

	scheduleReconnect() {
		this.reconnectTimer = setTimeout(async () => {
			try {
				await this.syncMissedMessages();
				this.connect();
			} catch {
				this.scheduleReconnect(); // Retry on failure
			}
		}, 3000); // 3 second backoff
	}

	async syncMissedMessages() {
		if (!this.lastKnownMessageId) return;

		try {
			const missedMessages = await ApiClient.request<Message[]>(
				`/api/v1/messages/sync?last_known_message_id=${this.lastKnownMessageId}`
			);
			for (const message of missedMessages) {
				messageStore.addMessage(message);
			}
		} catch (error) {
			logger.error('websocket', 'REST fallback sync failed', undefined, error as Error);
		}
	}

	onMessage(event: { type: string; id?: string }) {
		if (event.id) {
			this.lastKnownMessageId = event.id;
		}

		if (event.type === 'workspace.user.removed') {
			authState.invalidate();
		}
	}
}

export const wsClient = new WsClient();
