<script lang="ts">
  import { open_wad, load_hashtables, WadTree } from "rust";
  import Icon from "@iconify/svelte";

  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { Separator } from "$lib/components/ui/select";
  import { ScrollArea } from "$lib/components/ui/scroll-area";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb";

  import * as stores from "$lib/stores";

  import { onMount } from "svelte";
  import { writable, type Writable } from "svelte/store";
  let input: HTMLInputElement | null = null;

  onMount(async () => {
    input = document.getElementById("input") as any;
  });

  let wad: Writable<WadTree | null> = writable(null);
  let path: number[] = [];

  const ext_icons: Record<string, string> = {
    anm: "material-symbols:animation",
    bin: "mdi:file",
    bnk: "mdi:file",
    dds: "mdi:image",
    jpg: "mdi:image",
    luaobj: "mdi:file",
    mapgeo: "mdi:file",
    png: "mdi:image",
    preload: "mdi:file",
    scb: "mdi:cube-outline",
    sco: "mdi:cube-outline",
    skl: "mdi:skull",
    skn: "ion:body",
    stringtable: "mdi:text",
    svg: "ph:file-svg",
    tex: "mdi:image",
    wgeo: "mdi:file",
    wpk: "mdi:file",
  };

  let hashesLoaded = stores.hashtables();
</script>

<main class="h-full flex flex-col">
  <header class="flex items-center gap-1">
    <Input id="input" type="file" class="flex-shrink w-min" />
    <Button
      disabled={!$hashesLoaded}
      on:click={async () => {
        if (!input || !input.files || input.files.length < 1) return;
        try {
          $wad = await open_wad(input.files[0]);
          console.log(Array.from($wad.children).map((i) => $wad?.get(i)));
        } catch (e) {
          console.error("failed to open wad: ", e);
        }
      }}
    >
      Open Wad
    </Button>
    {#if !$hashesLoaded}
      <Icon icon="fluent:spinner-ios-16-filled" class="h-5 w-5 animate-spin" />
      <span class="text-muted-foreground text-sm">Loading hashtables...</span>
    {/if}
    <div class="flex flex-grow" />
  </header>

  <Separator />

  {#if $wad}
    {@const view =
      (path.length == 0
        ? $wad.children
        : $wad.get(path[path.length - 1])?.children) ?? []}

    <Breadcrumb.Root>
      <Breadcrumb.List class="gap-0 sm:gap-0">
        <Breadcrumb.Item>
          <Breadcrumb.Link asChild class="px-2 py-1" let:attrs>
            <button
              {...attrs}
              on:click|preventDefault={() => {
                path = [];
              }}>/</button
            >
          </Breadcrumb.Link>
        </Breadcrumb.Item>
        <Breadcrumb.Separator />
        {#each path as c, i}
          <Breadcrumb.Item>
            <Breadcrumb.Link asChild class="px-2 py-1" let:attrs>
              <button
                {...attrs}
                on:click|preventDefault={() => {
                  path = path.slice(0, i + 1);
                }}>{$wad.get(c)?.name}</button
              >
            </Breadcrumb.Link>
          </Breadcrumb.Item>
          {#if i < path.length - 1}
            <Breadcrumb.Separator />
          {/if}
        {/each}
      </Breadcrumb.List>
    </Breadcrumb.Root>

    <ScrollArea class="rounded-md h-full">
      <ul class="pb-5">
        {#if path.length > 0}
          <li>
            <button
              class="w-full p-2 text-left hover:bg-white/10 rounded-md"
              on:click={() => {
                path.pop();
                path = path;
              }}
              >..
            </button>
          </li>
        {/if}
        {#each view as i}
          {@const child = $wad.get(i)}
          {@const ext = child?.name.split(".").at(-1)}
          <li>
            <button
              class="w-full p-2 text-left hover:bg-white/10 flex items-center gap-1 rounded-md"
              on:click={() => {
                if (child?.is_dir()) {
                  path.push(i);
                  path = path;
                }
              }}
            >
              <Icon
                class="mt-1 w-4 h-4"
                icon={(ext && ext_icons[ext]) ||
                  (child?.children?.length ?? 0 > 0
                    ? "mdi:folder"
                    : "mdi:file")}
              />
              {child?.name}
            </button>
          </li>
        {/each}
      </ul>
    </ScrollArea>
  {/if}
</main>
