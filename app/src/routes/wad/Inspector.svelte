<script lang="ts">
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import * as context from '$lib/context';
	import { getPreviewKind, type PreviewKind } from '$lib/consts';

	import TexturePreview from './TexturePreview.svelte';
	import { type Component } from 'svelte';
	import StaticMeshPreview from './StaticMeshPreview.svelte';
	import type { Item } from '$lib/pkg/rust';

	let ctx = context.wad.get();
	let selected = $derived(ctx.selected !== null ? (ctx.wad?.get(ctx.selected) ?? null) : null);

	let previewer: { [K in PreviewKind]: Component<{ item: Item }> } = {
		texture: TexturePreview,
		mesh: StaticMeshPreview
	};
</script>

<ScrollArea>
	{#if ctx.wad}
		{#if selected !== null}
			{@const kind = getPreviewKind(selected)}
			{#if kind !== undefined}
				{@const Comp = previewer[kind]}
				<Comp item={selected} />
			{/if}
		{/if}
	{/if}
</ScrollArea>
