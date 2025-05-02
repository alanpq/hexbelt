<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import Icon from "@iconify/svelte";
  import { createEventDispatcher } from "svelte";

  const sort_icons = {
    asc: "mdi:sort-ascending",
    desc: "mdi:sort-descending",
  } as const;

  interface Props {
    order: keyof typeof sort_icons | undefined;
    children?: import('svelte').Snippet;
  }

  let { order, children }: Props = $props();

  const dispatch = createEventDispatcher<{ click: void }>();
</script>

<Button variant="ghost" class="" on:click={() => dispatch("click")}>
  {@render children?.()}
  {#if order}
    <Icon icon={sort_icons[order]} class="ml-2 h-4 w-4" />
  {/if}
</Button>
