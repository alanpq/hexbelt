<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLAttributes } from 'svelte/elements';

	let {
		class: className,
		children,
		child,
		hover = $bindable(false),
		onFiles,
		...restProps
	}: HTMLAttributes<HTMLElement> & {
		onFiles?: (files: DataTransfer['files']) => void;
		hover?: boolean;
		child?: Snippet<[{ props: typeof listeners }]>;
	} = $props();

	let listeners = {
		ondragenter: () => {
			hover = true;
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
			if (!onFiles) return;
			if (!e.dataTransfer) return;
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
