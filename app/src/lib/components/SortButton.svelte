<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import Icon from "@iconify/svelte";
  import { createEventDispatcher } from "svelte";

  const sort_icons = {
    asc: "mdi:sort-ascending",
    desc: "mdi:sort-descending",
  } as const;

  export let order: keyof typeof sort_icons | undefined;

  const dispatch = createEventDispatcher<{ click: void }>();
</script>

<Button variant="ghost" class="" on:click={() => dispatch("click")}>
  <slot />
  {#if order}
    <Icon icon={sort_icons[order]} class="ml-2 h-4 w-4" />
  {/if}
</Button>
