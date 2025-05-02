<script lang="ts">
	import FileDrop from '$lib/components/FileDrop.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Upload } from '@lucide/svelte';
	import { fade } from 'svelte/transition';

	let file: File | null = $state(null);
</script>

{#if file === null}
	<p class="flex flex-grow" transition:fade={{ duration: 100 }}>
		<FileDrop
			class="m-5 flex-grow"
			onFiles={(files) => {
				file = files.item(0);
			}}
		>
			<h2>No file open.</h2>
			<p class="text-sm text-muted-foreground">Drag and drop a file or</p>
			<Button>Upload<Upload /></Button>
		</FileDrop>
	</p>
{:else}{/if}
