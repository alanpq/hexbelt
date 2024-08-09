<script lang="ts">
  import { WadTree, Item } from "rust";
  import Icon from "@iconify/svelte";
  import { toast } from "svelte-sonner";
  import type { Writable } from "svelte/store";

  import { Button } from "$lib/components/ui/button";
  import Spinner from "$lib/components/Spinner.svelte";

  export let wad: WadTree;
  export let item: Item;

  let downloading = false;
</script>

{#if item.is_file()}
  <Button
    class="w-6 h-6 p-0"
    variant="ghost"
    on:click={() => {
      downloading = true;
      try {
        const data = wad.load_chunk_data(item.id);
        const url = URL.createObjectURL(new Blob([data.buffer]));
        const link = document.createElement("a");
        link.href = url;
        link.download = item.name;
        link.click();
      } catch (e) {
        console.error("Failed to download file", e);
        toast.error(`Failed to download file - ${e}`);
      }
      downloading = false;
    }}
  >
    {#if downloading}
      <Spinner class="w-4 h-4" />
    {:else}
      <Icon class="w-4 h-4" icon="mdi-download" />
    {/if}
  </Button>
{:else}
  <div aria-hidden="true" class="w-6 h-6" />
{/if}
