<script lang="ts">
	import { cn } from '$lib/utils';
	import type { HTMLAttributes } from 'svelte/elements';
	import FileDrop from './FileDrop.svelte';
	import type { Snippet } from 'svelte';
	import { fade } from 'svelte/transition';
	import { Upload } from '@lucide/svelte';
	let {
		class: className,
		innerClass,
		overlayClass,
		disabled = false,
		children,
		overlay,
		onFiles,
		...restProps
	}: HTMLAttributes<HTMLElement> & {
		disabled?: boolean;
		innerClass?: string;
		overlayClass?: string;
		onFiles?: (files: DataTransfer['files']) => void;
		overlay?: Snippet;
	} = $props();

	let hover = $state(false);
</script>

<FileDrop {onFiles} {disabled} bind:hover>
	{#snippet child({ props })}
		<div {...props} {...restProps} class={cn('relative size-full', className)}>
			<div class={cn('absolute inset-0', hover && 'pointer-events-none', innerClass)}>
				{@render children?.()}
			</div>
			{#if hover}
				<section
					transition:fade={{ duration: 100 }}
					class={cn(
						'pointer-events-none absolute inset-0 flex select-none flex-col items-center justify-center gap-2 rounded border border-dashed border-muted-foreground/30 bg-card/80 p-3 text-center transition-colors',
						hover && '',
						overlayClass
					)}
				>
					{#if overlay}
						{@render overlay()}
					{:else}
						<Upload class="text-primary" />
						<p class="text-sm">Drop to open file.</p>
					{/if}
				</section>
			{/if}
		</div>
	{/snippet}
</FileDrop>
