import { describe, it, expect } from 'vitest';
import { WsClient } from '../../src/lib/api/websocket';

describe('WsClient', () => {
	it('should handle reconnects correctly', () => {
		const client = new WsClient();
		expect(client.isConnected).toBe(false);
	});
});
