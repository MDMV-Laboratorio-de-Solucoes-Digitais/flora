import { beforeEach, describe, expect, it } from 'vitest';
import { FileStore } from '../../src/lib/state/FileStore.svelte';
import type { FileAttachment } from '../../src/lib/types/models';
import { filesQueryPath } from '../../src/lib/utils/files';

function makeFile(id: string, fileName: string): FileAttachment {
	return {
		id,
		fileName,
		mimeType: 'text/plain',
		byteSize: 128,
		rustFsUrl: `/api/v1/files/download/${id}`,
		uploadedAt: '2026-11-23T03:00:00Z'
	};
}

describe('filesQueryPath', () => {
	it('encodes workspace_id in the workspace-scoped files endpoint path', () => {
		expect(filesQueryPath('88888888-8888-8888-8888-888888888888')).toBe(
			'/api/v1/files?workspace_id=88888888-8888-8888-8888-888888888888'
		);
		expect(filesQueryPath('ws with spaces&x=1')).toBe(
			'/api/v1/files?workspace_id=ws%20with%20spaces%26x%3D1'
		);
	});
});

describe('FileStore', () => {
	let store: FileStore;

	beforeEach(() => {
		store = new FileStore();
	});

	it('replaces items with setFiles and resets with clear', () => {
		const initial = [makeFile('id-1', 'one.txt'), makeFile('id-2', 'two.txt')];
		store.setFiles(initial);
		expect(store.files).toEqual(initial);
		expect(store.files).not.toBe(initial);

		store.clear();
		expect(store.files).toEqual([]);
	});

	it('prepends newly uploaded files newest-first without duplicating existing IDs', () => {
		const older = makeFile('id-1', 'older.txt');
		const newer = makeFile('id-2', 'newer.txt');

		store.setFiles([older]);
		store.addFile(newer);
		store.addFile(older);

		expect(store.files).toEqual([newer, older]);
	});

	it('removes a file by id', () => {
		store.setFiles([makeFile('id-1', 'one.txt'), makeFile('id-2', 'two.txt')]);
		store.removeFile('id-1');
		expect(store.files.map((f) => f.id)).toEqual(['id-2']);
	});
});
