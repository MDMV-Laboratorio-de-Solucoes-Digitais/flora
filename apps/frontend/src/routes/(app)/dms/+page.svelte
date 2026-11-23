<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import {
		Card,
		CardContent,
		CardDescription,
		CardFooter,
		CardHeader,
		CardTitle
	} from '$lib/components/ui/card';
	import { t } from '$lib/i18n';
</script>

<div class="mx-auto flex w-full max-w-2xl flex-col gap-6 p-6">
	<h1 class="text-2xl font-bold">{t['dms.title']}</h1>

	<Card>
		<CardHeader>
			<CardTitle>{t['dms.unavailableTitle']}</CardTitle>
			<CardDescription>{t['dms.unavailableDescription']}</CardDescription>
		</CardHeader>
		<CardContent></CardContent>
		<CardFooter>
			<Button variant="outline" href="/channels">{t['dms.goToChannels']}</Button>
		</CardFooter>
	</Card>
</div>
