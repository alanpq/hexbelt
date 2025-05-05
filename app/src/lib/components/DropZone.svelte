<script lang="ts">
	import { cn } from '$lib/utils';
	import type { HTMLAttributes } from 'svelte/elements';
	import FileDrop from './FileDrop.svelte';
	let {
		class: className,
		children,
		onFiles,
		...restProps
	}: HTMLAttributes<HTMLElement> & { onFiles?: (files: DataTransfer['files']) => void } = $props();

	let hover = $state(false);
</script>

<FileDrop {onFiles} bind:hover>
	{#snippet child({ props })}
		<section
			{...props}
			{...restProps}
			class={cn(
				'flex select-none flex-col items-center justify-center gap-2 rounded border border-dashed border-muted-foreground/30 p-3 text-center transition-colors',
				hover && 'bg-foreground/5',
				className
			)}
		>
			{@render children?.()}
		</section>
	{/snippet}
</FileDrop>
