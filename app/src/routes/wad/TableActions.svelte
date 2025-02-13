<script lang="ts">
  import { WadTree, Item, Bin } from "$lib/pkg/rust";
  import Icon from "@iconify/svelte";
  import { toast } from "svelte-sonner";
  import type { Writable } from "svelte/store";

  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import { Button } from "$lib/components/ui/button";
  import Spinner from "$lib/components/Spinner.svelte";

  import * as stores from "$lib/stores";
  import { goto } from "$app/navigation";
  import { base } from "$app/paths";

  export let wad: WadTree;
  export let item: Item;

  let bin = stores.bin();

  let downloading = false;

  const load_chunk = (id: number) => {
    try {
      return wad.load_chunk_data(item.id);
    } catch (e) {
      console.error("Failed to download file", e);
      toast.error(`Failed to download file - ${e}`);
      return null;
    }
  };
</script>

{#if item.is_file()}
  <Button
    class="w-6 h-6 p-0"
    variant="ghost"
    on:click={() => {
      downloading = true;
      const data = load_chunk(item.id);
      if (!data) return;
      const url = URL.createObjectURL(new Blob([data.buffer]));
      const link = document.createElement("a");
      link.href = url;
      link.download = item.name;
      link.click();
      downloading = false;
    }}
  >
    {#if downloading}
      <Spinner class="w-4 h-4" />
    {:else}
      <Icon class="w-4 h-4" icon="mdi-download" />
    {/if}
  </Button>
  <DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
      <Button
        variant="ghost"
        builders={[builder]}
        size="icon"
        class="w-6 h-6 p-0"
      >
        <span class="sr-only">Open menu</span>
        <Icon icon="lucide:ellipsis" class="h-4 w-4" />
      </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content>
      <DropdownMenu.Item
        on:click={() => {
          const data = load_chunk(item.id);
          if (!data) return;
          try {
            $bin = Bin.from_bytes(data);
          } catch (e) {
            console.error(e);
            toast.error(`${e}`);
          }
          goto(base + "/bin");
        }}
      >
        Open in Bin Viewer
      </DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
{:else}
  <div aria-hidden="true" class="w-6 h-6" />
{/if}
