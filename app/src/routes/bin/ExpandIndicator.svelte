<script>
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import Icon from "@iconify/svelte";

  export let isExpanded;
  export let canExpand;
  export let isAllSubRowsExpanded;
  export let depth;
</script>

{#if $canExpand}
  <Button
    variant="ghost"
    on:click={() => ($isExpanded = !$isExpanded)}
    class="w-4 h-4 p-0"
  >
    <Icon
      icon={$isExpanded ? "lets-icons:expand-down" : "lets-icons:expand-right"}
      class="w-4 h-4"
    />
  </Button>
{/if}
