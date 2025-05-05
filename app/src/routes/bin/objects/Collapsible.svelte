<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { Data, TreeNode, BinEntryValue, BinEntry } from '$lib/pkg/rust';
	import { ChevronDown, ChevronRight, Folder } from '@lucide/svelte';
	import { onMount, type Component } from 'svelte';

	import * as context from '$lib/context';
	import Node from '../TreeNode.svelte';
	import { cn } from '$lib/utils';
	import Entry from '../Entry.svelte';

	let {
		tree,
		entry,
		name,
		parent,
		class: className
	}: { tree: Data; entry: BinEntry; name?: string; parent?: string; class?: string } = $props();

	let id = $derived(`${parent}/${name ?? entry.name}`);
	let ctx = context.bin.get();
	let expanded = $derived(
		(parent === undefined || ctx.expanded.has(id)) && entry.children.length > 0
	);
	let Icon = $derived(expanded ? ChevronDown : ChevronRight);

	let entries = entry.children;
</script>

<li class={cn('col-span-full row-span-1 grid grid-cols-subgrid items-center', className)}>
	{#if parent !== undefined}
		<Button
			variant="ghost"
			class="flex flex-row justify-start p-1 px-2 text-left"
			disabled={entry.children.length == 0}
			onclick={() => {
				if (expanded) ctx.expanded.delete(id);
				else ctx.expanded.add(id);
			}}
		>
			<Icon />
			<Folder />
			{entry.name}
		</Button>
		<span class="flex h-full items-center p-1 px-2 text-sm text-muted-foreground/50 hover:bg-card"
			>{entry.value.kind.replace('Property', '')}</span
		>
	{/if}
	{#if expanded}
		<ul
			class={cn(
				'col-span-full row-span-1 ml-2 grid grid-cols-subgrid',
				entries.length > 1 && 'border-l'
			)}
		>
			{#each entries as entry}
				<Entry {tree} {entry} parent={id} />
			{/each}
		</ul>
	{/if}
</li>
