<script lang="ts">
	import { decode_texture } from '$lib/pkg/rust';
	import { cn } from '$lib/utils';
	import { Slider } from '$lib/components/ui/slider';
	import { useDebounce } from 'runed';

	let { name, data, mipmap = 0 }: { name?: string; data?: Uint8Array; mipmap?: number } = $props();

	let max_mips = $state(1);

	let preview_canvas: HTMLCanvasElement = $state() as HTMLCanvasElement;
	let has_preview: boolean | string = $state(false);

	let mips: {
		image: ImageData;
		width: number;
		height: number;
		mipmaps: number;
	}[] = [];

	$effect(() => {
		data;
		mipmap;
		useDebounce(drawPreview, 100)();
	});

	$effect(() => {
		data;
		mips = [];
	});

	const drawPreview = () => {
		if (!preview_canvas || !data) return;
		const ctx = preview_canvas.getContext('2d');
		if (!mips[mipmap]) {
			const tex = decode_texture(data, mipmap);
			mips[mipmap] = {
				image: new ImageData(new Uint8ClampedArray(tex.data), tex.width, tex.height),
				width: tex.width,
				height: tex.height,
				mipmaps: tex.mipmaps
			};
		}
		const tex = mips[mipmap];
		max_mips = tex.mipmaps - 1;
		preview_canvas.width = tex.width;
		preview_canvas.height = tex.height;
		has_preview = true;
		ctx?.putImageData(tex.image, 0, 0);
	};
</script>

<div class="max-h-[80%] w-full">
	{#if typeof has_preview === 'string'}
		<section class="p-4 text-red-400">
			<h1 class="font-bold">Failed to load preview:</h1>
			{has_preview}
		</section>
	{/if}
	<div
		class={cn(
			'grid h-full w-full flex-grow grid-cols-1 grid-rows-[auto,min-content] content-center justify-center gap-1 font-mono text-sm',
			(has_preview !== true || !data) && 'hidden'
		)}
	>
		<canvas
			bind:this={preview_canvas}
			class="col-span-2 col-start-1 row-span-2 row-start-1 h-full w-full object-contain"
		></canvas>
		<span class="col-start-1 row-start-1 h-min w-min p-1 text-white mix-blend-difference"
			>{name ?? '??'}</span
		>
		{#if !!data && has_preview === true && preview_canvas}
			<span
				class="col-start-1 row-start-1 h-min place-self-end p-1 pb-0 text-right text-white mix-blend-difference"
				>{max_mips > 0 ? `mip ${mipmap} ` : ''}<br
				/>{preview_canvas.width}x{preview_canvas.height}</span
			>
		{/if}
		{#if max_mips > 0}
			<Slider
				type="single"
				class="col-span-2 mx-auto my-1 mb-3 max-w-[calc(100%-2em)]"
				min={0}
				max={max_mips}
				step={1}
				value={mipmap}
				onValueChange={(v) => {
					mipmap = v;
				}}
			/>
		{/if}
	</div>
</div>
