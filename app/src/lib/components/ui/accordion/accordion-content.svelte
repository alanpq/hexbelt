<script lang="ts">
	import { Accordion as AccordionPrimitive, type WithoutChild } from 'bits-ui';
	import { cn } from '$lib/utils.js';
	import type { Snippet } from 'svelte';

	let {
		ref = $bindable(null),
		class: className,
		child,
		children,
		...restProps
	}: WithoutChild<AccordionPrimitive.ContentProps> & {
		child?: Snippet;
	} = $props();
</script>

<AccordionPrimitive.Content
	bind:ref
	class={cn(
		'overflow-hidden text-sm data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down',
		className
	)}
	{...restProps}
>
	{#if child}
		{@render child()}
	{:else}
		<div class="pb-4 pt-0">
			{@render children?.()}
		</div>
	{/if}
</AccordionPrimitive.Content>
