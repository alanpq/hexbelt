<script lang="ts">
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import DropZone from '$lib/components/DropZone.svelte';
	import * as Card from '$lib/components/ui/card';
	import UploadButton from '$lib/components/UploadButton.svelte';
	import * as context from '$lib/context';

	const wadCtx = context.wad.get();
	const openWad = (files: FileList) => {
		context.openWad(wadCtx, files);
		goto(`${base}/wad`);
	};
	const binCtx = context.bin.get();
	const openBin = (files: FileList) => {
		context.openBin(binCtx, files);
		goto(`${base}/bin`);
	};
</script>

<article class="m-auto grid w-full max-w-[50ch] grid-cols-2 gap-4">
	<section class="col-span-2 mb-8 flex flex-col gap-0 text-center">
		<h1 class="text-6xl tracking-tighter">hexbelt</h1>
		<h2 class="text-muted-foreground">Useful tools for League of Legends custom skin creation.</h2>
	</section>
	<Card.Root class="flex flex-col">
		<Card.Header>
			<Card.Title>Wad Walker</Card.Title>
			<Card.Description>Open & extract from .wad files</Card.Description>
		</Card.Header>
		<span class="flex-grow"></span>
		<DropZone class="m-5" onFiles={openWad}>
			<p class="text-sm text-muted-foreground">Drop a file or</p>
			<UploadButton onFiles={openWad} />
		</DropZone>
	</Card.Root>
	<Card.Root class="flex flex-col">
		<Card.Header>
			<Card.Title>Bin Explorer</Card.Title>
			<Card.Description>Browse .bin fields</Card.Description>
		</Card.Header>
		<span class="flex-grow"></span>
		<DropZone class="m-5" onFiles={openBin}>
			<p class="text-sm text-muted-foreground">Drop a file or</p>
			<UploadButton onFiles={openBin} />
		</DropZone>
	</Card.Root>
	<p class="col-span-2 text-center text-sm text-muted-foreground">& more to come :3</p>
</article>
