<script lang="ts">
	import { cn, normToRgba } from '$lib/utils';
	import type { ValueColor } from './process';

	let { color }: { color?: ValueColor } = $props();

	let hasSomeColor = $derived(color?.constantValue || color?.dynamics);

	const toCSS = (col: [number, number, number, number]) => `rgba(${normToRgba(col).join(',')})`;
</script>

<section
	class={cn('size-full rounded', !hasSomeColor && 'border border-dashed')}
	style={color &&
		hasSomeColor &&
		`background: ${
			color.dynamics
				? `linear-gradient(to left, ${color.dynamics.values.map((col, i) => `${toCSS(col)} ${(color.dynamics?.times[i] ?? 0) * 100}%`).join(',')})`
				: toCSS(color.constantValue ?? [1, 0, 1, 1])
		}`}
></section>
