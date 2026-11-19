import { execFileSync } from 'node:child_process';
import path from 'node:path';

const ROOT = path.resolve(import.meta.dirname, '../../..');

export default function globalSetup(): void {
	execFileSync(
		'cargo',
		['build', '-q', '-p', 'flora-core', '--bin', 'flora-mint-session', '--features', 'dev-mint'],
		{ cwd: ROOT, stdio: 'inherit' }
	);
}
