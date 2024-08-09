<script lang="ts">
  import { Item } from "rust";

  import { createEventDispatcher } from "svelte";
  import Icon from "@iconify/svelte";
  import ListItem from "./ListItem.svelte";

  export let child: Item;
  const dispatch = createEventDispatcher<{
    click: void;
  }>();

  $: ext = child.name.split(".").at(-1);

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
  {child?.name}
</ListItem>
