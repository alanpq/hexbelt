<script lang="ts">
  import { decode_texture, type Texture } from "$lib/pkg/rust";
  import { cn } from "$lib/utils";
  import debounce from "lodash/debounce";

  export let name: string | undefined = undefined;
  export let data: Uint8Array | undefined = undefined;
  export let mipmap: number = 0;

  let max_mips = 1;

  let preview_canvas: null | HTMLCanvasElement = null;
  let has_preview: boolean | string = false;

  let mips: {
    image: ImageData;
    width: number;
    height: number;
    mipmaps: number;
  }[] = [];

  $: {
    data;
    mipmap;
    debounce(drawPreview, 100)();
  }

  $: {
    data;
    mips = [];
  }

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
</script>

<div class="w-full max-h-[100%]">
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
    />
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
      <input
        class="col-span-2 mx-2"
        type="range"
        min="0"
        max={max_mips}
        step="1"
        bind:value={mipmap}
      />
    {/if}
  </div>
</div>
