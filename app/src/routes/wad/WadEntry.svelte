<script lang="ts">
  import { Item } from "$lib/pkg/rust";

  import { createEventDispatcher } from "svelte";
  import Icon from "@iconify/svelte";
  import ListItem from "./ListItem.svelte";
  import Button from "$lib/components/ui/button/button.svelte";
  import Spinner from "$lib/components/Spinner.svelte";

  interface Props {
    child: Item;
    download?: boolean;
  }

  let { child, download = false }: Props = $props();
  const dispatch = createEventDispatcher<{
    click: void;
    download: () => void;
  }>();

  let ext = $derived(child.name.split(".").at(-1));

  let downloading = $state(false);

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
</script>

<ListItem
  icon={(ext && ext_icons[ext]) ||
    (child.children?.length ?? 0 > 0 ? "mdi:folder" : "mdi:file")}
  on:click={() => dispatch("click")}
>
  <span class="flex-grow">{child?.name}</span>
  {#if download}
    <Button
      class="w-6 h-6 p-0"
      variant="ghost"
      on:click={() => {
        downloading = true;
        dispatch("download", () => {
          downloading = false;
        });
      }}
    >
      {#if downloading}
        <Spinner class="w-4 h-4" />
      {:else}
        <Icon class="w-4 h-4" icon="mdi-download" />
      {/if}
    </Button>
  {/if}
</ListItem>
