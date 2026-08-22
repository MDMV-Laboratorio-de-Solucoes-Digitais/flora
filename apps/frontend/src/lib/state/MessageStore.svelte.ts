import type { Message } from '../types/models';

export class MessageStore {
	messages = $state<Message[]>([]);

	addMessage(message: Message) {
		if (this.messages.every((m) => m.id !== message.id)) {
			this.messages.push(message);
		}
	}
}
export const messageStore = new MessageStore();
