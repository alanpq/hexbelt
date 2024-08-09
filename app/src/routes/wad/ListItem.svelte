<script lang="ts">
  import Icon from "@iconify/svelte";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{
    click: void;
  }>();
  export let icon: string | undefined = undefined;
</script>

<button
  class="w-full p-2 text-left hover:bg-white/10 flex items-center gap-1 rounded-md"
  on:click={() => dispatch("click")}
>
  {#if icon}
    <Icon class="mt-1 w-4 h-4" {icon} />
  {/if}
  <slot />
</button>
