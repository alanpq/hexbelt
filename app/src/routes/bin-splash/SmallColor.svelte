<script lang="ts">
	import { cn, normToRgba } from '$lib/utils';
	import type { ValueColor } from './process';

	let { color }: { color?: ValueColor } = $props();
	let value = $derived(color?.constantValue);
</script>

<section
	class={cn('size-full rounded', !value && 'border border-dashed')}
	style={value && `background-color: rgba(${normToRgba(value).join(',')})`}
></section>
