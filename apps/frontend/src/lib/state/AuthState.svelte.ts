export class AuthState {
	isAuthenticated = $state(false);
	user = $state<Record<string, unknown> | null>(null);
	token = $state<string | null>(null);
	isSessionLocked = $state(false);
	isInvalidated = $state(false);
	gracePeriodTimeout: ReturnType<typeof setTimeout> | null = null;

	login(token: string, user: Record<string, unknown>) {
		this.token = token;
		this.user = user;
		this.isAuthenticated = true;
		this.isSessionLocked = false;
		if (this.gracePeriodTimeout) {
			clearTimeout(this.gracePeriodTimeout);
			this.gracePeriodTimeout = null;
		}
	}

	logout() {
		this.token = null;
		this.user = null;
		this.isAuthenticated = false;
		this.isSessionLocked = false;
		if (this.gracePeriodTimeout) {
			clearTimeout(this.gracePeriodTimeout);
			this.gracePeriodTimeout = null;
		}
	}

	lockSession() {
		if (this.isSessionLocked) return;
		this.isSessionLocked = true;

		// 5 minute grace period before full logout
		this.gracePeriodTimeout = setTimeout(
			() => {
				this.logout();
			},
			5 * 60 * 1000
		);
	}

	invalidate() {
		this.isInvalidated = true;
		this.logout();
	}
}

export const authState = new AuthState();
