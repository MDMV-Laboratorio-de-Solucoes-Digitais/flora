import type { FileAttachment } from '../types/models';

export class FileStore {
	files = $state<FileAttachment[]>([]);

	setFiles(files: FileAttachment[]) {
		this.files = [...files];
	}

	clear() {
		this.files = [];
	}

	addFile(file: FileAttachment) {
		if (this.files.every((f) => f.id !== file.id)) {
			this.files = [file, ...this.files];
		}
	}

	removeFile(fileId: string) {
		this.files = this.files.filter((f) => f.id !== fileId);
	}
}
export const fileStore = new FileStore();
