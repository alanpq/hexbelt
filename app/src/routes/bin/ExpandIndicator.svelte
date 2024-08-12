<script>
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import Icon from "@iconify/svelte";

  export let name;
  export let isExpanded;
  export let canExpand;
  export let isAllSubRowsExpanded;
  export let depth;
</script>

<Button
  variant="ghost"
  disabled={!$canExpand}
  on:click={() => ($isExpanded = !$isExpanded)}
  class="min-w-[10ch] h-full p-1 flex justify-start gap-1"
>
  <Icon
    icon={$isExpanded ? "lets-icons:expand-down" : "lets-icons:expand-right"}
    class={cn("w-4 h-4", !$canExpand && "opacity-0")}
  />
  <span>{name}</span>
</Button>
