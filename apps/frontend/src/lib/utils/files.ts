export function isValidFileSize(sizeBytes: number): boolean {
	const MAX_SIZE = 50 * 1024 * 1024;
	return sizeBytes <= MAX_SIZE;
}
