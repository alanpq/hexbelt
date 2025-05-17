<script lang="ts">
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Input } from '$lib/components/ui/input';
	import type { BinEntryValue } from '$lib/pkg/rust';
	import { cn } from '$lib/utils';

	const is_vec = (t: (typeof value)['value']): t is { kind: string; value: number[] } => {
		return t.kind.slice(0, 6) == 'Vector';
	};

	const is_color = (t: number[]): t is [number, number, number, number] => t.length == 4;

	const containerClasses: Record<any, string | undefined> = {
		String: 'text-primary'
	};

	let {
		value,
		class: className
	}: { value: Extract<BinEntryValue, { kind: 'PropertyJSValue' }>; class?: string } = $props();

	let inner = $derived(value.value);
	let isVec = $derived(is_vec(inner));
	let isNum = $derived(['U', 'I', 'F'].indexOf(inner.kind[0]) != -1);

	let colorClass = $derived.by(() => {
		const cc = containerClasses[inner.kind];
		if (cc) return cc;
		if (isNum || isVec) return 'text-cyan-500';
		return null;
	});
</script>

<span
	class={cn(
		'grid h-full grid-flow-col items-center gap-1 px-1 py-0.5 transition-colors',
		// !isVec && 'hover:bg-card',
		colorClass,
		className
	)}
>
	{#if inner.kind == 'String'}
		<Input value={inner.value} autocomplete="off" />
	{:else if isNum}
		<Input value={inner.value} autocomplete="off" />
	{:else if inner.kind == 'Bool' || inner.kind == 'BitBool'}
		<Checkbox value={inner.value} />
	{:else if inner.kind == 'Hash'}
		<Input value={inner.value.toString(16)} autocomplete="off" />
	{:else if inner.kind == 'ObjectLink'}
		<Input value={inner.value.toString(16)} autocomplete="off" />
	{:else if inner.kind == 'Color'}
		<!-- <Color color={inner.value} /> -->
	{:else if is_vec(inner)}
		{#each inner.value as i}
			<Input
				type="text"
				class="overflow-clip text-ellipsis font-mono"
				autocomplete="off"
				value={i.toString().length < 4 ? i.toPrecision(4) : i}
			/>
		{/each}
	{:else}
		Unknown js '{inner.kind}' - {JSON.stringify(inner.value)}
	{/if}
</span>
