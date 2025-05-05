<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { Data, TreeNode, BinEntryValue, BinEntry } from '$lib/pkg/rust';
	import { ChevronDown, ChevronRight, Folder } from '@lucide/svelte';
	import type { Component } from 'svelte';

	import * as context from '$lib/context';
	import Node from '../TreeNode.svelte';
	import { cn } from '$lib/utils';
	import Entry from '../Entry.svelte';
	import Optional from './Optional.svelte';

	import { Checkbox } from '$lib/components/ui/checkbox';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Input } from '$lib/components/ui/input';

	let {
		tree,
		entry,
		parent,
		class: className
	}: {
		tree: Data;
		entry: BinEntry & { value: Extract<BinEntryValue, { kind: 'PropertyJSValue' }> };
		parent?: string;
		class?: string;
	} = $props();

	const is_vec = (t: (typeof entry)['value']['value']): t is { kind: string; value: number[] } => {
		return t.kind.slice(0, 6) == 'Vector';
	};

	const is_color = (t: number[]): t is [number, number, number, number] => t.length == 4;

	const containerClasses: Record<any, string | undefined> = {
		String: 'text-primary'
	};

	let inner = $derived(entry.value.value);
	let isVec = $derived(is_vec(inner));
	let isNum = $derived(['U', 'I', 'F'].indexOf(inner.kind[0]) != -1);

	let colorClass = $derived.by(() => {
		const cc = containerClasses[inner.kind];
		if (cc) return cc;
		if (isNum) return 'text-cyan-500';
		return null;
	});
</script>

<li class="col-span-full grid min-h-9 grid-cols-subgrid items-center text-sm">
	<span
		class="flex h-full items-center p-1 px-2 text-muted-foreground transition-colors hover:bg-card"
	>
		{entry.name}
	</span>
	<span
		class={cn(
			'flex h-full items-center p-1 px-2 text-muted-foreground transition-colors hover:bg-card'
			// colorClass,
			// colorClass && 'opacity-50'
		)}
	>
		{inner.kind}
	</span>
	<span
		class={cn(
			'grid h-full grid-flow-col items-center gap-2 px-1 py-0.5 transition-colors',
			// !isVec && 'hover:bg-card',
			colorClass
		)}
	>
		{#if inner.kind == 'String'}
			<!-- "{inner.value}" -->
			<Input value={inner.value} />
		{:else if isNum}
			<Input value={inner.value} />
		{:else if inner.kind == 'Bool' || inner.kind == 'BitBool'}
			<Checkbox value={inner.value} />
		{:else if inner.kind == 'Hash'}
			>0x{inner.value.toString(16)}
		{:else if inner.kind == 'ObjectLink'}
			0x{inner.value.toString(16)}
		{:else if inner.kind == 'Color'}
			<!-- <Color color={inner.value} /> -->
		{:else if is_vec(inner)}
			{#each inner.value as i}
				<Input
					type="text"
					class="overflow-clip text-ellipsis px-1.5 font-mono text-cyan-300"
					value={i.toString().length < 4 ? i.toPrecision(4) : i}
				/>
			{/each}
		{:else}
			Unknown js '{inner.kind}' - {JSON.stringify(inner.value)}
		{/if}
	</span>
</li>
