<script lang="ts">
	import { cn, normToRgba } from '$lib/utils';
	import type { ValueColor } from './process';

	import * as Popover from '$lib/components/ui/popover';

	let { color }: { color?: ValueColor } = $props();

	let hasSomeColor = $derived(color?.constantValue || color?.dynamics);

	const toCSS = (col: [number, number, number, number]) => `rgba(${normToRgba(col).join(',')})`;

	const rgbToHex = (color: number[]) => {
		return '#' + color.map((c) => (c * 255).toString(16).padStart(2, '0')).join('');
	};
	const fromHex = (hex: string): [number, number, number] => {
		return [hex.slice(1, 3), hex.slice(3, 5), hex.slice(5, 7)].map(
			(c) => parseInt(c, 16) / 255
		) as [number, number, number];
	};
</script>

<Popover.Root>
	<Popover.Trigger
		class={cn('size-full rounded', !hasSomeColor && 'border border-dashed')}
		style={color &&
			hasSomeColor &&
			`background: ${
				color.dynamics
					? `linear-gradient(to left, ${color.dynamics.values.map((col, i) => `${toCSS(col)} ${(color.dynamics?.times[i] ?? 0) * 100}%`).join(',')})`
					: toCSS(color.constantValue ?? [1, 0, 1, 1])
			}`}
	></Popover.Trigger>
	<Popover.Content>
		{#if color && color.constantValue}
			<input
				type="color"
				value={rgbToHex(color.constantValue.slice(0, 3))}
				onchange={(e) => {
					color.constantValue = [...fromHex(e.currentTarget.value), 1];
				}}
			/>
		{/if}
	</Popover.Content>
</Popover.Root>
