<script lang="ts">
	import type { Snippet } from 'svelte';

	import { cn } from '$lib/utils';
	import type { HTMLAttributes } from 'svelte/elements';

	let {
		children,
		selected = false,
		disabled = false,
		directory = false,
		class: className,
		...restProps
	}: {
		children?: Snippet;
		selected?: boolean;
		directory?: boolean;
		disabled?: boolean;
	} & HTMLAttributes<HTMLButtonElement> = $props();
</script>

<button
	{disabled}
	class={cn(
		'col-span-full row-span-1 grid cursor-default grid-cols-subgrid grid-rows-subgrid items-center rounded p-1 text-left transition-colors disabled:cursor-default',
		!disabled && 'hover:bg-primary/20',
		disabled && 'text-muted-foreground',
		selected && 'bg-primary hover:bg-primary',
		className
	)}
	{...restProps}
>
	{@render children?.()}
</button>
