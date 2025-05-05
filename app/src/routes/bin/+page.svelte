<script lang="ts">
	import DropZone from '$lib/components/DropZone.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Sidebar from '$lib/components/ui/sidebar';

	import { Upload } from '@lucide/svelte';
	import { fade } from 'svelte/transition';

	import * as context from '$lib/context';
	import { open_bin } from '$lib/pkg/rust';
	import TreeNode from './TreeNode.svelte';

	let ctx = context.bin.get();

	let root = $derived(ctx.bin?.data.tree);
</script>

{#if !(root !== undefined && ctx.bin !== null)}
	<Sidebar.Trigger />
	<div class="flex flex-grow" out:fade={{ duration: 100 }}>
		<DropZone
			class="m-5 flex-grow"
			onFiles={async (files) => {
				ctx.bin = await open_bin(files[0]);
			}}
		>
			<h2>No file open.</h2>
			<p class="text-sm text-muted-foreground">Drag and drop a file or</p>
			<Button>Upload<Upload /></Button>
		</DropZone>
	</div>
{:else}
	<section class="" in:fade={{ delay: 100, duration: 100 }}>
		<ul class="grid grid-cols-[max-content_10ch_1fr] items-center">
			<li
				class="col-span-full grid grid-cols-subgrid items-center gap-4 pb-1 pt-[1px] text-sm font-bold"
			>
				<h2 class="flex items-center">
					<Sidebar.Trigger class="-mt-[1px]" />
					<span class="pl-2"> Key </span>
				</h2>
				<h2>Type</h2>
				<h2>Value</h2>
			</li>
			<TreeNode node={root} tree={ctx.bin.data} />
		</ul>
	</section>
{/if}
