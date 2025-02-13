<script lang="ts">
  import { open_wad, decode_texture, Item } from "$lib/pkg/rust";
  import Icon from "@iconify/svelte";

  import { Separator } from "$lib/components/ui/select";
  import { ScrollArea } from "$lib/components/ui/scroll-area";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb";
  import * as Resizable from "$lib/components/ui/resizable";

  import * as stores from "$lib/stores";

  import { writable } from "svelte/store";
  import { toast } from "svelte-sonner";
  import Table from "./Table.svelte";
  import FilePicker from "$lib/components/FilePicker.svelte";
  import { browser } from "$app/environment";
  import { cn } from "$lib/utils";

  let wad = stores.wad();
  let path = stores.wad_path();

  let selected: null | Item = null;
  let preview_canvas: null | HTMLCanvasElement = null;
  let has_preview: boolean | string = false;

  let view = writable<Item[]>([]);
  path.subscribe((path) => {
    if (!$wad) return;
    $view = (
      path.length == 0
        ? Array.from($wad.children)
        : Array.from($wad.get(path[path.length - 1])?.children ?? [])
    )
      .map((i) => $wad?.get(i))
      .filter((c) => !!c);
  });

  let hashesLoaded = stores.wad_hashtables();
  let isCollapsed = browser ? !!localStorage.getItem("collapsed") : false;

  let defaultLayout = (browser
    ? JSON.parse(localStorage.getItem("wad_layout") || "0")
    : null) || [265, 440];

  const onLayoutChange = (sizes: number[]) => {
    if (!browser) return;
    localStorage.setItem("wad_layout", JSON.stringify(sizes));
  };

  const drawPreview = (id: number) => {
    if (!preview_canvas || !$wad) return;
    const data = $wad.load_chunk_data(id);
    const tex = decode_texture(data);
    const ctx = preview_canvas.getContext("2d");
    preview_canvas.width = tex.width;
    preview_canvas.height = tex.height;
    has_preview = true;
    ctx?.putImageData(
      new ImageData(new Uint8ClampedArray(tex.data), tex.width, tex.height),
      0,
      0,
    );
  };
</script>

<main class="h-full flex flex-col">
  <header class="flex items-center gap-1">
    <FilePicker
      class="flex-shrink w-min"
      disabled={!$hashesLoaded}
      on:open={async ({ detail: files }) => {
        try {
          $wad = await open_wad(files[0]);
          console.log(Array.from($wad.children).map((i) => $wad?.get(i)));
          $path = [];
        } catch (e) {
          console.error("failed to open wad: ", e);
          toast.error(`Failed to open Wad! ${e}`);
        }
      }}>Open Wad</FilePicker
    >
    {#if !$hashesLoaded}
      <Icon icon="fluent:spinner-ios-16-filled" class="h-5 w-5 animate-spin" />
      <span class="text-muted-foreground text-sm">Loading hashtables...</span>
    {/if}
    <div class="flex flex-grow" />
  </header>

  <Separator />

  {#if $wad}
    <Resizable.PaneGroup
      direction="horizontal"
      {onLayoutChange}
      class="h-full w-full items-stretch"
    >
      <Resizable.Pane defaultSize={defaultLayout[1]} minSize={30}>
        <Breadcrumb.Root>
          <Breadcrumb.List class="gap-0 sm:gap-0">
            <Breadcrumb.Item>
              <Breadcrumb.Link asChild class="px-2 py-1" let:attrs>
                <button
                  {...attrs}
                  on:click|preventDefault={() => {
                    $path = [];
                  }}>/</button
                >
              </Breadcrumb.Link>
            </Breadcrumb.Item>
            <Breadcrumb.Separator />
            {#each $path as c, i}
              <Breadcrumb.Item>
                <Breadcrumb.Link asChild class="px-2 py-1" let:attrs>
                  <button
                    {...attrs}
                    on:click|preventDefault={() => {
                      $path = $path.slice(0, i + 1);
                    }}>{$wad.get(c)?.name}</button
                  >
                </Breadcrumb.Link>
              </Breadcrumb.Item>
              {#if i < $path.length - 1}
                <Breadcrumb.Separator />
              {/if}
            {/each}
          </Breadcrumb.List>
        </Breadcrumb.Root>

        <ScrollArea class="rounded-md h-full">
          <Table
            wad={$wad}
            {path}
            data={view}
            on:select={(e) => {
              if (!$wad || !preview_canvas) return;
              selected = e.detail !== null ? $wad.get(e.detail) ?? null : null;
              if (selected !== null && selected.is_file()) {
                const ext = selected.name.split(".").at(-1);
                if (ext === "dds" || ext === "tex") {
                  try {
                    drawPreview(selected.id);
                  } catch (e) {
                    has_preview = `${e}`;
                  }
                }
              } else {
                has_preview = false;
              }
            }}
          />
        </ScrollArea>
      </Resizable.Pane>
      <Resizable.Handle withHandle />
      <Resizable.Pane
        class={cn("flex flex-col place-items-center justify-center")}
        defaultSize={defaultLayout[0]}
        collapsible
        minSize={5}
      >
        {#if typeof has_preview === "string"}
          <section class="text-red-400 p-4">
            <h1 class="font-bold">Failed to load preview:</h1>
            {has_preview}
          </section>
        {/if}
        <div
          class={cn(
            "grid grid-cols-2 gap-1 grid-rows-[min-content,min-content] h-min justify-center content-center flex-grow w-full font-mono text-sm",
            has_preview !== true && "hidden",
          )}
        >
          <canvas
            bind:this={preview_canvas}
            class="w-full col-span-2 object-contain"
          />
          {#if selected && has_preview === true && preview_canvas}
            <span class="pl-2">{selected.name}</span>
            <span class="pr-2 text-right"
              >{preview_canvas.width}x{preview_canvas.height}</span
            >
          {/if}
        </div>
      </Resizable.Pane>
    </Resizable.PaneGroup>
  {/if}
</main>
