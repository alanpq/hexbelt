<script lang="ts">
  import { cn } from "$lib/utils";
  import Icon from "@iconify/svelte";

  let className: string = "";
  export { className as class };
</script>

<Icon
  icon="fluent:spinner-ios-16-filled"
  class={cn("h-5 w-5 animate-spin", className)}
/>
