import type { FileAttachment } from '../types/models';

export class FileStore {
	files = $state<FileAttachment[]>([]);

	addFile(file: FileAttachment) {
		if (this.files.every((f) => f.id !== file.id)) {
			this.files.push(file);
		}
	}

	removeFile(fileId: string) {
		this.files = this.files.filter((f) => f.id !== fileId);
	}
}
export const fileStore = new FileStore();
