<script lang="ts">
	import type { Data, BinEntryValue, BinEntry } from '$lib/pkg/rust';

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
	<Entry {tree} entry={{ ...entry, value: entry.value.value }} {parent} class={className} />
{/if}
