<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLAttributes } from 'svelte/elements';

	let {
		class: className,
		disabled = false,
		children,
		child,
		hover = $bindable(false),
		onFiles,
		...restProps
	}: HTMLAttributes<HTMLElement> & {
		onFiles?: (files: DataTransfer['files']) => void;
		disabled?: boolean;
		hover?: boolean;
		child?: Snippet<[{ props: typeof listeners }]>;
	} = $props();

	let listeners = {
		ondragenter: (e: DragEvent) => {
			if (
				!e.dataTransfer ||
				Array.from(e.dataTransfer.items).find((item) => item.kind === 'file') === undefined
			)
				return;
			hover = true && !disabled;
		},
		ondragexit: () => {
			hover = false;
		},
		ondragover: (e: Event) => {
			e.preventDefault();
		},
		ondrop: (e: DragEvent) => {
			e.preventDefault();
			hover = false;
			if (disabled || !onFiles || !e.dataTransfer) return;
			if (e.dataTransfer.files.length == 0) return;
			onFiles(e.dataTransfer.files);
		}
	};
</script>

{#if child}
	{@render child({ props: listeners })}
{:else}
	<div {...restProps} {...listeners}>
		{@render children?.()}
	</div>
{/if}
