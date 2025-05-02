<script lang="ts">
  import { cn } from "$lib/utils";
  import Icon from "@iconify/svelte";

  interface Props {
    class?: string;
  }

  let { class: className = "" }: Props = $props();
  
</script>

<Icon
  icon="fluent:spinner-ios-16-filled"
  class={cn("h-5 w-5 animate-spin", className)}
/>
