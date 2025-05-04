<script lang="ts">
	import { Download, Eye, File as FileIcon, Folder, Table2, Undo2, Upload } from '@lucide/svelte';

	import FileDrop from '$lib/components/FileDrop.svelte';
	import TableEntry from './TableEntry.svelte';

	import { Button } from '$lib/components/ui/button';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import TooltipContent from '$lib/components/ui/tooltip/tooltip-content.svelte';

	import * as context from '$lib/context';

	import type { Component } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-sonner';

	import { open_wad } from '$lib/pkg/rust';

	let ctx: ReturnType<typeof context.wad.get> = $state({ wad: null, path: [] });
	ctx = context.wad.getOr({ wad: null, path: [] });

	let selected = $state<number | null>(null);

	const file_icons: Record<any, Component> = {
		bin: Table2
	};

	let view = $derived.by(() => {
		if (!ctx.wad) return [];
		return Array.from(
			ctx.path.length == 0 ? ctx.wad.children : (ctx.wad.get(ctx.path.at(-1) ?? -1)?.children ?? [])
		)
			.map((i) => ctx.wad?.get(i))
			.filter((c) => !!c);
	});

	let file: File | null = $state(null);
</script>

{#if !ctx.wad}
	<div class="flex flex-grow" out:fade={{ duration: 100 }}>
		<FileDrop
			class="m-5 flex-grow"
			onFiles={async (files) => {
				file = files.item(0);
				try {
					ctx.wad = null;
					ctx.wad = await open_wad(files[0]);
					if (!ctx.wad) throw new Error('failed to open wad');
					console.log(Array.from(ctx.wad.children).map((i) => ctx.wad?.get(i)));
					ctx.path = [];
				} catch (e) {
					console.error('failed to open wad: ', e);
					toast.error(`Failed to open Wad! ${e}`);
				}
			}}
		>
			<h2>No file open.</h2>
			<p class="text-sm text-muted-foreground">Drag and drop a file or</p>
			<Button>Upload<Upload /></Button>
		</FileDrop>
	</div>
{:else}
	<section class="mt-5 flex-grow">
		<ul
			in:fade={{ delay: 100, duration: 100 }}
			class="grid grid-cols-[min-content,1fr] items-center gap-x-2"
		>
			<li class="contents">
				<TableEntry
					directory
					disabled={ctx.path.length == 0}
					onclick={() => {
						ctx.path.splice(-1);
					}}
					class="cursor-pointer"
				>
					<Undo2 class="size-4" />
					...
				</TableEntry>
			</li>
			{#each view as item}
				{@const is_dir = item.is_dir()}
				{@const ext = item.name.split('.').at(-1)}
				{@const Icon = (ext !== undefined && file_icons[ext]) || (is_dir ? Folder : FileIcon)}
				<li class="contents">
					<ContextMenu.Root
						onOpenChange={(open) => {
							if (!open) return;
							selected = item.id;
						}}
					>
						<ContextMenu.Trigger>
							{#snippet child({ props })}
								<TableEntry
									{...props}
									selected={item.id === selected}
									directory={is_dir}
									onclick={() => {
										selected = item.id;
									}}
									ondblclick={() => {
										if (is_dir) {
											ctx.path.push(item.id);
										}
									}}
								>
									<Icon class="size-4" />
									{item.name}
								</TableEntry>
							{/snippet}
						</ContextMenu.Trigger>
						<ContextMenu.Content>
							<ContextMenu.Item class="flex gap-2" disabled={is_dir}>
								<Eye class="size-4" /> Preview
							</ContextMenu.Item>
							<ContextMenu.Item class="flex gap-2">
								<Download class="size-4" /> Download
							</ContextMenu.Item>
						</ContextMenu.Content>
					</ContextMenu.Root>
				</li>
			{/each}
		</ul>
	</section>
{/if}
