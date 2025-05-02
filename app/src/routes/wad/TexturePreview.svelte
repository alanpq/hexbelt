<script lang="ts">
  import { run } from 'svelte/legacy';

  import { decode_texture, type Texture } from "$lib/pkg/rust";
  import { cn } from "$lib/utils";
  import { Slider } from "$lib/components/ui/slider";
  import debounce from "lodash/debounce";

  interface Props {
    name?: string | undefined;
    data?: Uint8Array | undefined;
    mipmap?: number;
  }

  let { name = undefined, data = undefined, mipmap = $bindable(0) }: Props = $props();

  let max_mips = $state(1);

  let preview_canvas: null | HTMLCanvasElement = $state(null);
  let has_preview: boolean | string = $state(false);

  let mips: {
    image: ImageData;
    width: number;
    height: number;
    mipmaps: number;
  }[] = $state([]);



  const drawPreview = () => {
    if (!preview_canvas || !data) return;
    const ctx = preview_canvas.getContext("2d");
    if (!mips[mipmap]) {
      const tex = decode_texture(data, mipmap);
      mips[mipmap] = {
        image: new ImageData(
          new Uint8ClampedArray(tex.data),
          tex.width,
          tex.height,
        ),
        width: tex.width,
        height: tex.height,
        mipmaps: tex.mipmaps,
      };
    }
    const tex = mips[mipmap];
    max_mips = tex.mipmaps - 1;
    preview_canvas.width = tex.width;
    preview_canvas.height = tex.height;
    has_preview = true;
    ctx?.putImageData(tex.image, 0, 0);
  };
  run(() => {
    data;
    mipmap;
    debounce(drawPreview, 100)();
  });
  run(() => {
    data;
    mips = [];
  });
</script>

<div class="w-full max-h-[80%]">
  {#if typeof has_preview === "string"}
    <section class="text-red-400 p-4">
      <h1 class="font-bold">Failed to load preview:</h1>
      {has_preview}
    </section>
  {/if}
  <div
    class={cn(
      "grid grid-cols-1 gap-1 grid-rows-[auto,min-content] h-full justify-center content-center flex-grow w-full font-mono text-sm",
      (has_preview !== true || !data) && "hidden",
    )}
  >
    <canvas
      bind:this={preview_canvas}
      class="w-full h-full row-start-1 col-start-1 row-span-2 col-span-2 object-contain"
></canvas>
    <span
      class="row-start-1 col-start-1 w-min h-min p-1 text-white mix-blend-difference"
      >{name ?? "??"}</span
    >
    {#if !!data && has_preview === true && preview_canvas}
      <span
        class="row-start-1 col-start-1 place-self-end p-1 pb-0 h-min text-right text-white mix-blend-difference"
        >{max_mips > 0 ? `mip ${mipmap} ` : ""}<br
        />{preview_canvas.width}x{preview_canvas.height}</span
      >
    {/if}
    {#if max_mips > 0}
      <Slider
        class="col-span-2 my-1 mb-3 w-[calc(100%-2em)] mx-auto"
        min={0}
        max={max_mips}
        step={1}
        value={[mipmap]}
        onValueChange={(v) => {
          mipmap = v[0];
        }}
      />
    {/if}
  </div>
</div>
