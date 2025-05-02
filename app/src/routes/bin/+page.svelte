<script lang="ts">
	import FileDrop from '$lib/components/FileDrop.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Upload } from '@lucide/svelte';

	let file: File | null = $state(null);
</script>

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
