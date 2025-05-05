<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { Data, TreeNode, BinEntryValue, BinEntry } from '$lib/pkg/rust';
	import { ChevronDown, ChevronRight, Folder } from '@lucide/svelte';
	import type { Component } from 'svelte';

	import * as context from '$lib/context';
	import Node from '../TreeNode.svelte';
	import { cn } from '$lib/utils';
	import Entry from '../Entry.svelte';

	let {
		tree,
		entry,
		parent,
		class: className
	}: {
		tree: Data;
		entry: BinEntry & { value: Extract<BinEntryValue, { kind: 'PropertyOptional' }> };
		parent?: string;
		class?: string;
	} = $props();
</script>

{#if entry.value.value}
	<Entry {tree} entry={{ ...entry, value: entry.value.value }} class={className} />
{/if}
