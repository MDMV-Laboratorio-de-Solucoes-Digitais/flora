<script lang="ts">
	import ChatLayout from '$lib/components/chat/ChatLayout.svelte';
	import { t } from '$lib/i18n';
</script>

<div class="p-6">
	<h1 class="mb-4 text-2xl font-bold">{t['dms.title']}</h1>
	<!-- For MVP, we just use a hardcoded DM channel ID -->
	<ChatLayout channelId="dm-channel-1" />
</div>
