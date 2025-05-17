<script lang="ts">
	import type { Data, BinEntryValue, BinEntry } from '$lib/pkg/rust';

	import { cn } from '$lib/utils';

	import Primitive from './Primitive.svelte';

	let {
		tree,
		entry,
		class: className
	}: {
		tree: Data;
		entry: BinEntry & { value: Extract<BinEntryValue, { kind: 'PropertyJSValue' }> };
		class?: string;
	} = $props();

	let inner = $derived(entry.value.value);
</script>

<li class={cn('col-span-full grid min-h-9 grid-cols-subgrid items-center text-sm', className)}>
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
	<Primitive value={entry.value} />
</li>
