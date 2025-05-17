<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { Data, TreeNode, BinEntryValue, BinEntry } from '$lib/pkg/rust';
	import {
		Boxes,
		ChevronDown,
		ChevronRight,
		Folder,
		ListOrdered,
		List,
		TableProperties,
		Braces
	} from '@lucide/svelte';
	import { onMount, type Component } from 'svelte';

	import * as context from '$lib/context';
	import Node from '../TreeNode.svelte';
	import { cn } from '$lib/utils';
	import Entry from '../Entry.svelte';
	import Primitive from './Primitive.svelte';

	let {
		tree,
		entry,
		key,
		parent,
		class: className
	}: {
		tree: Data;
		entry: BinEntry;
		key?: BinEntryValue;
		parent?: string;
		class?: string;
	} = $props();

	let id = $derived(`${parent}/${entry.name}`);
	let ctx = context.bin.get();
	let expanded = $derived(
		(parent === undefined || ctx.expanded.has(id)) && entry.children.length > 0
	);
	let CollapseIcon = $derived(expanded ? ChevronDown : ChevronRight);

	const kindIcons: { [K in BinEntryValue['kind']]?: Component } = {
		PropertyStruct: Boxes,
		PropertyEmbedded: Boxes,
		PropertyContainer: ListOrdered,
		PropertyUnorderedContainer: List,
		PropertyMap: TableProperties,

		Object: Braces
	};

	let KindIcon = $derived(kindIcons[entry.value.kind] ?? Folder);

	let entries = entry.children;
</script>

<li class={cn('col-span-full row-span-1 grid grid-cols-subgrid items-center', className)}>
	{#if parent !== undefined}
		{#if key}
			<span class="flex flex-row items-center gap-0">
				<Button
					variant="ghost"
					class="flex flex-row place-items-center gap-1 p-1 px-2 "
					disabled={entry.children.length == 0}
					onclick={() => {
						if (expanded) ctx.expanded.delete(id);
						else ctx.expanded.add(id);
					}}
				>
					<CollapseIcon />
				</Button>
				{#if key.kind == 'PropertyJSValue'}
					<Primitive value={key} class={className} />
				{:else}
					{JSON.stringify(key)}
				{/if}
				{#if entries.length > 1}
					<span class="text-right text-sm tracking-tighter text-muted-foreground">
						({entries.length})
					</span>
				{/if}
			</span>
		{:else}
			<Button
				variant="ghost"
				class="flex flex-row justify-start gap-1 p-1 px-2 pl-1 text-left"
				disabled={entry.children.length == 0}
				onclick={() => {
					if (expanded) ctx.expanded.delete(id);
					else ctx.expanded.add(id);
				}}
			>
				<CollapseIcon />
				{entry.name}
				{#if entries.length > 1}
					<span class="text-right text-sm tracking-tighter text-muted-foreground">
						({entries.length})
					</span>
				{/if}
			</Button>
		{/if}
		<span class="flex h-full items-center p-1 px-2 text-sm text-muted-foreground/50 hover:bg-card">
			<KindIcon class="mr-1 size-4" />
			{'value' in entry.value && 'className' in entry.value.value && entry.value.value.className
				? entry.value.value.className
				: entry.value.kind.replace('Property', '')}
		</span>
	{/if}
	{#if expanded}
		<ul
			class={cn(
				'col-span-full row-span-1 ml-2.5 grid grid-cols-subgrid',
				entries.length > 1 && 'border-l'
			)}
		>
			{#each entries as entry}
				<Entry {tree} {entry} parent={id} />
			{/each}
		</ul>
	{/if}
</li>
