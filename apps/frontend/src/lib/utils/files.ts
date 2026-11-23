/** Seed default in `flora-core/src/config/storage.rs` (`max_upload_bytes`). */
export const MAX_FILE_SIZE_BYTES = 10 * 1024 * 1024;

export function isValidFileSize(sizeBytes: number): boolean {
	return sizeBytes <= MAX_FILE_SIZE_BYTES;
}

export function filesQueryPath(workspaceId: string): string {
	return `/api/v1/files?workspace_id=${encodeURIComponent(workspaceId)}`;
}
