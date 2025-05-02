<script lang="ts">
	import { goto } from '$app/navigation';
	import FileDrop from '$lib/components/FileDrop.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { Upload } from '@lucide/svelte';
</script>

<article class="m-auto grid w-full max-w-[50ch] grid-cols-2 gap-4">
	<section class="col-span-2 mb-8 flex flex-col gap-0 text-center">
		<h1 class=" text-6xl tracking-tighter">hexbelt</h1>
		<h2 class="text-muted-foreground">Useful tools for League of Legends custom skin creation.</h2>
	</section>
	<Card.Root class="flex flex-col">
		<Card.Header>
			<Card.Title>Wad Walker</Card.Title>
			<Card.Description>Open & extract from .wad files</Card.Description>
		</Card.Header>
		<span class="flex-grow"></span>
		<FileDrop
			class="m-5"
			onFiles={(files) => {
				goto('/wad');
			}}
		>
			<p class="text-sm text-muted-foreground">Upload or drop a file</p>
			<Button size="icon"><Upload /></Button>
		</FileDrop>
	</Card.Root>
	<Card.Root class="flex flex-col">
		<Card.Header>
			<Card.Title>Bin Explorer</Card.Title>
			<Card.Description>Browse .bin fields</Card.Description>
		</Card.Header>
		<span class="flex-grow"></span>
		<FileDrop
			class="m-5"
			onFiles={(files) => {
				goto('/bin');
			}}
		>
			<p class="text-sm text-muted-foreground">Upload or drop a file</p>
			<Button size="icon"><Upload /></Button>
		</FileDrop>
	</Card.Root>
	<p class="col-span-2 text-center text-sm text-muted-foreground">& more to come :3</p>
</article>
