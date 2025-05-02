<script lang="ts">
	import { cn } from '$lib/utils';
	import type { HTMLAttributes } from 'svelte/elements';
	let {
		class: className,
		children,
		onFiles,
		...restProps
	}: HTMLAttributes<HTMLElement> & { onFiles?: (files: DataTransfer['files']) => void } = $props();

	let hover = $state(false);
</script>

<section
	{...restProps}
	class={cn(
		'flex select-none flex-col items-center gap-2 rounded border border-dashed border-muted-foreground/30 p-3 text-center transition-colors',
		hover && 'bg-foreground/5',
		className
	)}
	ondragenter={() => {
		hover = true;
	}}
	ondragexit={() => {
		hover = false;
	}}
	ondragover={(e) => {
		e.preventDefault();
	}}
	ondrop={(e) => {
		e.preventDefault();
		hover = false;
		if (!onFiles) return;
		if (!e.dataTransfer) return;
		if (e.dataTransfer.files.length == 0) return;
		onFiles(e.dataTransfer.files);
	}}
>
	{@render children?.()}
</section>
