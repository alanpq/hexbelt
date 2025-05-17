<script lang="ts">
	import type { Snippet } from 'svelte';
	import Button from './ui/button/button.svelte';
	import { Upload } from '@lucide/svelte';

	let {
		children,
		class: className,
		onFiles
	}: { children?: Snippet; class?: string; onFiles?: (files: FileList) => void } = $props();

	let files: FileList | null | undefined = $state();
	let input = $state() as HTMLInputElement;
</script>

<Button
	class={className}
	onclick={() => {
		input.click();
	}}
>
	{#if children}
		{@render children()}
	{:else}
		Upload<Upload />
	{/if}
</Button>
<input
	bind:this={input}
	bind:files
	class="hidden"
	aria-hidden="true"
	type="file"
	onchange={() => {
		if (files === null || files === undefined || files.length == 0) return;
		onFiles?.(files);
	}}
/>
