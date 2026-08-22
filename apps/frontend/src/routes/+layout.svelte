<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { authState } from '$lib/state/AuthState.svelte';
	import { wsClient } from '$lib/api/websocket';

	let { children } = $props();

	$effect(() => {
		if (authState.isAuthenticated && !wsClient.isConnected) {
			wsClient.connect();
		} else if (!authState.isAuthenticated && wsClient.isConnected) {
			wsClient.disconnect();
		}

		if (authState.isInvalidated) {
			// Redirect to login flow
			location.assign('/');
		}
	});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

{#if authState.isInvalidated}
	<div class="session-lock-overlay">
		<div class="session-lock-modal">
			<h2>Access Revoked</h2>
			<p>You have been removed from this workspace.</p>
		</div>
	</div>
{:else if authState.isSessionLocked}
	<div class="session-lock-overlay">
		<div class="session-lock-modal">
			<h2>Session Expired</h2>
			<p>
				Your session has expired. You are in read-only mode for 5 minutes before automatic logout.
			</p>
			<button onclick={() => authState.logout()}>Logout Now</button>
		</div>
	</div>
{/if}

<div class={authState.isSessionLocked ? 'read-only-mode' : ''}>
	{@render children()}
</div>

<style>
	.session-lock-overlay {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 9999;
		pointer-events: none; /* Let clicks pass through to the background if we want true read-only, 
		                         but actually the spec says "UI overlay behavior". 
								 If it's an overlay that blocks, we shouldn't have pointer-events: none. 
								 Let's make it block interactions. */
	}

	.session-lock-overlay {
		pointer-events: auto;
	}

	.session-lock-modal {
		background: white;
		padding: 2rem;
		border-radius: 8px;
		text-align: center;
		color: black;
		pointer-events: auto;
	}

	.read-only-mode {
		opacity: 0.8;
		pointer-events: none; /* Disable all interactions with the app */
	}
</style>
