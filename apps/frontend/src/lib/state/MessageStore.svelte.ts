import type { Message } from '../types/models';

function byTimeline(a: Message, b: Message): number {
	const at = Date.parse(a.createdAt);
	const bt = Date.parse(b.createdAt);
	if (at !== bt && !Number.isNaN(at) && !Number.isNaN(bt)) return at - bt;
	return a.id.localeCompare(b.id);
}

export class MessageStore {
	messages = $state<Message[]>([]);

	setMessages(messages: Message[]) {
		this.messages = [...messages].toSorted(byTimeline);
	}

	clear() {
		this.messages = [];
	}

	upsertMessage(message: Message) {
		const index = this.messages.findIndex((m) => m.id === message.id);
		if (index === -1) {
			this.messages = [...this.messages, message].toSorted(byTimeline);
			return;
		}
		const next = [...this.messages];
		next[index] = { ...next[index], ...message };
		this.messages = next.toSorted(byTimeline);
	}

	removeMessage(id: string) {
		this.messages = this.messages.filter((m) => m.id !== id);
	}

	addMessage(message: Message) {
		// Back-compat alias used by older callers; idempotent upsert.
		this.upsertMessage(message);
	}
}

export const messageStore = new MessageStore();
