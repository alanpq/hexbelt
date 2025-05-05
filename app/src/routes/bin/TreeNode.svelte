<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { Data, TreeNode, BinEntryValue } from '$lib/pkg/rust';
	import { ChevronDown, ChevronRight, Folder } from '@lucide/svelte';
	import type { Component } from 'svelte';

	import * as context from '$lib/context';
	import Self from './TreeNode.svelte';
	import { cn } from '$lib/utils';
	import * as objects from './objects';
	import Entry from './Entry.svelte';

	let {
		tree,
		node,
		parent,
		class: className
	}: { tree: Data; node: TreeNode; parent?: string; class?: string } = $props();

	let id = $derived(`${parent}/${node.value[0]}`);
	let ctx = context.bin.get();
	let expanded = $derived(parent === undefined || ctx.expanded.has(id));
	let Icon = $derived(expanded ? ChevronDown : ChevronRight);
</script>

{#if node.kind === 'Namespace'}
	{@const [name, keys] = node.value}
	{@const entries = Object.entries(keys)}
	<li class={cn('col-span-full row-span-1 grid grid-cols-subgrid items-center', className)}>
		{#if parent !== undefined}
			<Button
				variant="ghost"
				class="flex flex-row justify-start gap-1 p-1 px-2 pl-1 text-left"
				onclick={() => {
					if (expanded) ctx.expanded.delete(id);
					else ctx.expanded.add(id);
				}}
			>
				<Icon />
				<Folder class="mr-1" />
				{name}
			</Button>
			<span
				class="flex h-full items-center p-1 px-2 text-sm text-muted-foreground/50 hover:bg-card"
			>
				Namespace
			</span>
			<span
				class="flex h-full items-center p-1 px-2 text-sm text-muted-foreground/50 hover:bg-card"
			>
				{entries.length > 0 ? `${entries.length} item${entries.length > 1 ? 's' : ''}` : 'empty'}
			</span>
		{/if}
		{#if expanded}
			<ul
				class={cn(
					'col-span-full row-span-1 ml-2.5 grid grid-cols-subgrid',
					entries.length > 1 && 'border-l'
				)}
			>
				{#each entries as [key, node]}
					<Self {tree} {node} parent={id} />
				{/each}
			</ul>
		{/if}
	</li>
{:else}
	{@const entry = tree.objects[node.value[1]]}
	<Entry {entry} {tree} parent={id} />
{/if}
