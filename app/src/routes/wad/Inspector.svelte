<script lang="ts">
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import * as context from '$lib/context';
	import { isPreviewable } from '$lib/consts';

	import TexturePreview from './TexturePreview.svelte';

	let ctx = context.wad.get();
	let selected = $derived(ctx.selected !== null ? (ctx.wad?.get(ctx.selected) ?? null) : null);
</script>

<ScrollArea>
	{#if ctx.wad}
		{#if selected !== null}
			{#if isPreviewable(selected)}
				{@const data = ctx.wad.load_chunk_data(selected.id)}
				<TexturePreview name={selected.name} {data} />
			{/if}
		{/if}
	{/if}
</ScrollArea>
