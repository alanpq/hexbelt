<script lang="ts">
	import DropZone from '$lib/components/DropZone.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Sidebar from '$lib/components/ui/sidebar';

	import { Upload } from '@lucide/svelte';
	import { fade } from 'svelte/transition';

	import * as context from '$lib/context';
	import TreeNode from './TreeNode.svelte';
	import DropOverlay from '$lib/components/DropOverlay.svelte';
	import UploadButton from '$lib/components/UploadButton.svelte';

	let ctx = context.bin.get();

	let root = $derived(ctx.bin?.data.tree);
	let opening = $state(false);

	const onFiles = async (files: FileList) => {
		context.openBin(ctx, files);
	};
</script>

{#if (root === undefined || ctx.bin === null) && !opening}
	<main class="flex w-full flex-col p-5" out:fade={{ duration: 100 }}>
		<Sidebar.Trigger />
		<div class="flex flex-grow">
			<DropZone class="m-5 flex-grow" {onFiles}>
				<h2>No file open.</h2>
				<p class="text-sm text-muted-foreground">Drag and drop a file or</p>
				<UploadButton {onFiles} />
			</DropZone>
		</div>
	</main>
{:else}
	<main class="flex w-full flex-col p-5 pr-0" in:fade={{ delay: 100, duration: 100 }}>
		<section class="flex-grow">
			<DropOverlay {onFiles} class="mr-5 flex">
				<ul
					class="grid max-h-full grid-cols-[max-content_max-content_1fr] grid-rows-[max-content_minmax(0,1fr)] items-center pr-5"
				>
					<li
						class="col-span-full grid grid-cols-subgrid items-center gap-4 pb-1 pt-[1px] text-sm font-bold shadow-background"
					>
						<h2 class="flex items-center">
							<Sidebar.Trigger class="-mt-[1px]" />
							<span class="pl-2"> Key </span>
						</h2>
						<h2>Type</h2>
						<h2>Value</h2>
					</li>
					{#if ctx.bin && root}
						<TreeNode node={root} tree={ctx.bin.data} class="max-h-full overflow-y-auto" />
					{/if}
				</ul>
			</DropOverlay>
		</section>
	</main>
{/if}
