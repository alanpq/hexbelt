<script lang="ts">
  import { open_wad, Item } from "rust";
  import Icon from "@iconify/svelte";

  import { Separator } from "$lib/components/ui/select";
  import { ScrollArea } from "$lib/components/ui/scroll-area";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb";

  import * as stores from "$lib/stores";

  import { writable } from "svelte/store";
  import { toast } from "svelte-sonner";
  import Table from "./Table.svelte";
  import FilePicker from "$lib/components/FilePicker.svelte";

  let wad = stores.wad();
  let path = stores.wad_path();

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
      <Table wad={$wad} {path} data={view} />
    </ScrollArea>
  {/if}
</main>
