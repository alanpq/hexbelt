<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import Icon from "@iconify/svelte";

  import type { BinEntry } from "$lib/pkg/rust";
  import { get_type } from "./types";

  export let value: BinEntry;
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
  {#if value.value.kind == "PropertyMapEntry" && "value" in value.value.value.key && !!value.value.value.key.value}
    <svelte:component
      this={get_type(value.value.value.key.kind)}
      value={value.value.value.key}
    />
  {:else}
    <span>{value.name}</span>
  {/if}
</Button>
