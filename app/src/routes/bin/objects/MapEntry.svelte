<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import type { Data, BinEntryValue, BinEntry } from '$lib/pkg/rust';
	import { cn } from '$lib/utils';

	import Entry from '../Entry.svelte';
	import CollapsibleS from './Collapsible.svelte';
	import Primitive from './Primitive.svelte';

	let {
		tree,
		entry,
		parent,
		class: className
	}: {
		tree: Data;
		entry: BinEntry & { value: Extract<BinEntryValue, { kind: 'PropertyMapEntry' }> };
		parent?: string;
		class?: string;
	} = $props();

	let inner = $derived(entry.value.value);

	let id = $derived(`${parent}/${entry.name}`);
</script>

{#snippet safePrimitive(value: BinEntryValue, className?: string)}
	{#if value.kind == 'PropertyJSValue'}
		<Primitive {value} class={className} />
	{:else}
		{JSON.stringify(value)}
	{/if}
{/snippet}

<li class={cn('col-span-full grid min-h-9 grid-cols-subgrid items-center text-sm', className)}>
	{#if inner.value.kind == 'PropertyJSValue'}
		<span
			class="flex h-full items-center px-1 py-0.5 text-muted-foreground transition-colors hover:bg-card"
		>
			{@render safePrimitive(inner.key, 'p-0')}
		</span>
		<span
			class={cn(
				'flex h-full items-center p-1 px-2 text-muted-foreground transition-colors hover:bg-card'
			)}
		>
			{inner.value.kind.replace('Property', '')}
		</span>
		<span class={cn('grid h-full grid-flow-col items-center gap-1 px-1 py-0.5 transition-colors')}>
			<Primitive value={inner.value} />
		</span>
	{:else}
		<CollapsibleS entry={{ ...entry, value: inner.value }} {tree} parent={id} />
		<!--
		{@const entries = entry.children}
		<Collapsible.Root>
			<Collapsible.Trigger
				>{#snippet child({ props })}
					<Button {...props}>+</Button>
				{/snippet}
			</Collapsible.Trigger>
			<Collapsible.Content>
				{#snippet child({ props })}
					<ul
						{...props}
						class={cn(
							'col-span-full row-span-1 ml-2.5 grid grid-cols-subgrid',
							entries.length > 1 && 'border-l'
						)}
					>
						{#each entries as entry}
							<Entry {tree} {entry} />
						{/each}
					</ul>
				{/snippet}</Collapsible.Content
			>
		</Collapsible.Root>-->
	{/if}
</li>
