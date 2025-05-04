<script lang="ts">
	import {
		Box,
		Download,
		Eye,
		File as FileIcon,
		FileImage,
		Folder,
		LetterText,
		PersonStanding,
		SendToBack,
		Skull,
		Table2,
		Undo2,
		Image as ImageIcon,
		Upload
	} from '@lucide/svelte';

	import FileDrop from '$lib/components/FileDrop.svelte';
	import TableEntry from './TableEntry.svelte';

	import { Button } from '$lib/components/ui/button';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import TooltipContent from '$lib/components/ui/tooltip/tooltip-content.svelte';

	import * as context from '$lib/context';

	import type { Component } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-sonner';

	import { Item, open_wad } from '$lib/pkg/rust';

	let ctx = context.wad.get();

	const file_icons: Record<any, Component> = {
		anm: SendToBack,
		bin: Table2,
		dds: ImageIcon,
		jpg: ImageIcon,
		png: ImageIcon,
		scb: Box,
		sco: Box,
		skl: Skull,
		skn: PersonStanding,
		stringtable: LetterText,
		svg: FileImage,
		tex: ImageIcon
	};

	const load_chunk = (id: number) => {
		try {
			if (!ctx.wad) throw new Error('Wad not loaded!');
			return ctx.wad.load_chunk_data(id);
		} catch (e) {
			console.error('Failed to download file', e);
			toast.error(`Failed to download file - ${e}`);
			return null;
		}
	};
	const download = (item: Item) => {
		const data = load_chunk(item.id);
		if (!data) return;
		const url = URL.createObjectURL(
			new Blob([data.buffer as BlobPart], { type: 'application/octet-stream' })
		);
		const link = document.createElement('a');
		link.href = url;
		link.download = item.name;
		link.click();
	};

	const previewable = new Set(['dds', 'jpg', 'png', 'tex', 'svg']);

	let view = $derived.by(() => {
		if (!ctx.wad) return [];
		return Array.from(
			ctx.path.length == 0 ? ctx.wad.children : (ctx.wad.get(ctx.path.at(-1) ?? -1)?.children ?? [])
		)
			.map((i) => ctx.wad?.get(i))
			.filter((c) => !!c);
	});
</script>

<header class="flex flex-row gap-4 items-center">
	<Sidebar.Trigger />
	{#if ctx.wad}
		<Breadcrumb.Root>
			<Breadcrumb.List>
				<Breadcrumb.Item>
					<Breadcrumb.Link
						class="cursor-pointer"
						onclick={() => {
							ctx.path = [];
						}}>/</Breadcrumb.Link
					>
				</Breadcrumb.Item>
				{#each ctx.path as path, i}
					<Breadcrumb.Separator />
					<Breadcrumb.Item>
						<Breadcrumb.Link
							class="cursor-pointer"
							onclick={() => {
								ctx.path.splice(i + 1);
							}}>{ctx.wad.get(path)?.name}</Breadcrumb.Link
						>
					</Breadcrumb.Item>
				{/each}
			</Breadcrumb.List>
		</Breadcrumb.Root>
	{/if}
</header>

{#if !ctx.wad}
	<div class="flex flex-grow" out:fade={{ duration: 100 }}>
		<FileDrop
			class="m-5 flex-grow"
			onFiles={async (files) => {
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
							ctx.selected = item.id;
						}}
					>
						<ContextMenu.Trigger>
							{#snippet child({ props })}
								<TableEntry
									{...props}
									selected={item.id === ctx.selected}
									directory={is_dir}
									onclick={() => {
										ctx.selected = item.id;
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
							<!-- TODO: file previews -->
							<ContextMenu.Item
								class="flex gap-2"
								disabled={is_dir || !ext || !previewable.has(ext)}
							>
								<Eye class="size-4" /> Preview
							</ContextMenu.Item>
							<!-- TODO: folder downloading -->
							<ContextMenu.Item
								class="flex gap-2"
								disabled={is_dir}
								onclick={() => {
									download(item);
								}}
							>
								<Download class="size-4" /> Download
							</ContextMenu.Item>
						</ContextMenu.Content>
					</ContextMenu.Root>
				</li>
			{/each}
		</ul>
	</section>
{/if}
