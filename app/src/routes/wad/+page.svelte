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
		Upload,
		X
	} from '@lucide/svelte';

	import DropZone from '$lib/components/DropZone.svelte';
	import TableEntry from './TableEntry.svelte';

	import { Button } from '$lib/components/ui/button';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import * as Resizable from '$lib/components/ui/resizable';
	import TooltipContent from '$lib/components/ui/tooltip/tooltip-content.svelte';

	import Inspector from './Inspector.svelte';

	import * as context from '$lib/context';
	import { previewableExtensions } from '$lib/consts';

	import type { Component } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-sonner';

	import { Bin, Item, open_wad } from '$lib/pkg/rust';
	import FileDrop from '$lib/components/FileDrop.svelte';
	import DropOverlay from '$lib/components/DropOverlay.svelte';
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { browser } from '$app/environment';
	import { cn } from '$lib/utils';
	import { ScrollArea } from '$lib/components/ui/scroll-area';

	let ctx = context.wad.get();
	let bin_ctx = context.bin.get();
	let opening = $state(false);

	let browserOpen = $derived(ctx.wad || opening);

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

	const onFiles = async (files: FileList) => {
		opening = true;
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
		opening = false;
	};

	let view = $derived.by(() => {
		if (!ctx.wad) return [];
		return Array.from(
			ctx.path.length == 0 ? ctx.wad.children : (ctx.wad.get(ctx.path.at(-1) ?? -1)?.children ?? [])
		)
			.map((i) => ctx.wad?.get(i))
			.filter((c) => !!c);
	});

	let defaultLayout = (() => {
		let sizes = (browser ? JSON.parse(localStorage.getItem('layout') || '0') : null) || [265, 440];
		if (!sizes[1] || sizes[1] < 5) sizes[1] = 440;
		return sizes;
	})();
	let inspectorOpen = $state(browser ? !!localStorage.getItem('wad_inspector') : true);
	const onLayoutChange = (sizes: number[]) => {
		if (!browser) return;
		localStorage.setItem('layout', JSON.stringify(sizes));
	};
	const onCollapse = () => {
		inspectorOpen = false;
		if (browser) localStorage.removeItem('wad_inspector');
	};
	const onExpand = () => {
		inspectorOpen = true;
		if (browser) localStorage.setItem('wad_inspector', '1');
	};
</script>

<main class="w-full">
	<DropOverlay {onFiles} disabled={!browserOpen} innerClass="" overlayClass="m-5">
		<Resizable.PaneGroup
			direction="horizontal"
			{onLayoutChange}
			class="h-full w-full items-stretch"
		>
			<Resizable.Pane
				defaultSize={defaultLayout[0]}
				minSize={10}
				class="flex w-full flex-col p-5 pr-2"
			>
				<header class="flex flex-row items-center gap-4">
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

				{#if !browserOpen}
					<div
						class="flex flex-grow"
						in:fade={{ delay: 100, duration: 100 }}
						out:fade={{ duration: 100 }}
					>
						<DropZone class="m-5 flex-grow" {onFiles}>
							<h2>No file open.</h2>
							<p class="text-sm text-muted-foreground">Drag and drop a file or</p>
							<Button>Upload<Upload /></Button>
						</DropZone>
					</div>
				{:else}
					<ScrollArea class="mt-5 flex-grow pr-4">
						<ul
							in:fade={{ delay: 100, duration: 100 }}
							out:fade={{ duration: 100 }}
							class="grid grid-cols-[min-content,minmax(0,1fr)] items-center gap-x-2"
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
								{@const Icon =
									(ext !== undefined && file_icons[ext]) || (is_dir ? Folder : FileIcon)}
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
													<span class="truncate"> {item.name}</span>
												</TableEntry>
											{/snippet}
										</ContextMenu.Trigger>
										<ContextMenu.Content>
											<!-- TODO: file previews -->
											<ContextMenu.Item
												class="flex gap-2"
												disabled={is_dir || !ext || !previewableExtensions.has(ext)}
												onclick={() => {
													inspectorOpen = true;
													onExpand();
												}}
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
											{#if ext == 'bin'}
												<ContextMenu.Separator />
												<ContextMenu.Item
													class="flex gap-2"
													onclick={() => {
														const data = load_chunk(item.id);
														if (!data) return;
														try {
															bin_ctx.bin = Bin.from_bytes(data);
														} catch (e) {
															console.error(e);
															toast.error(`${e}`);
														}
														goto(base + '/bin');
													}}
												>
													<Table2 class="size-4" /> Open in Bin Explorer
												</ContextMenu.Item>
											{/if}
										</ContextMenu.Content>
									</ContextMenu.Root>
								</li>
							{/each}
						</ul>
					</ScrollArea>
				{/if}
			</Resizable.Pane>
			{#if inspectorOpen}
				<Resizable.Handle withHandle />
				<Resizable.Pane
					class={cn('')}
					defaultSize={defaultLayout[1]}
					minSize={5}
					collapsible
					collapsedSize={1}
					{onCollapse}
					{onExpand}
				>
					<header class="flex flex-row p-5">
						<span class="flex-grow"></span>
						<Button
							variant="ghost"
							size="icon"
							class="size-7"
							onclick={() => {
								inspectorOpen = false;
								onCollapse();
							}}><X /></Button
						>
					</header>
					<Inspector />
				</Resizable.Pane>
			{/if}
		</Resizable.PaneGroup>
	</DropOverlay>
</main>
