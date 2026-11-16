import type { Message } from '../types/models';

export class MessageStore {
	messages = $state<Message[]>([]);

	setMessages(messages: Message[]) {
		this.messages = messages;
	}

	clear() {
		this.messages = [];
	}

	addMessage(message: Message) {
		if (this.messages.every((m) => m.id !== message.id)) {
			this.messages = [...this.messages, message];
		}
	}
}

export const messageStore = new MessageStore();
