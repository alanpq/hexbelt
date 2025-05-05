<script lang="ts">
	import type { Data, BinEntryValue, BinEntry } from '$lib/pkg/rust';
	import type { Component } from 'svelte';

	import * as objects from './objects';

	const obj_types: {
		[K in BinEntryValue['kind']]?: Component<{
			tree: Data;
			entry: any;
			name?: string;
			parent?: string;
			class?: string;
		}>;
	} = {
		Object: objects.Collapsible,
		Namespace: undefined,
		PropertyJSValue: objects.JSValue,
		PropertyNone: undefined,
		PropertyOptional: objects.Optional,
		PropertyContainer: objects.Collapsible,
		PropertyUnorderedContainer: objects.Collapsible,
		PropertyMap: undefined,
		PropertyMapEntry: undefined,
		PropertyStruct: objects.Collapsible,
		PropertyEmbedded: objects.Collapsible
	};

	let {
		tree,
		entry,
		name,
		parent,
		class: className
	}: { tree: Data; entry: BinEntry; name?: string; parent?: string; class?: string } = $props();

	let EntryComponent = $derived(obj_types[entry.value.kind]);
</script>

{#if EntryComponent !== undefined}
	<EntryComponent {tree} {entry} {name} {parent} class={className} />
{:else}
	<li class="col-span-full">
		{entry.value.kind}
	</li>
{/if}
