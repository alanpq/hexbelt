<script lang="ts">
  import Icon from "@iconify/svelte";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{
    click: void;
  }>();
  export let icon: string | undefined = undefined;
  export let disabled: boolean = false;
</script>

<button
  class="w-full p-2 text-left hover:bg-white/10 flex items-center gap-1 rounded-md disabled:text-muted-foreground disabled:hover:bg-transparent"
  on:click={() => dispatch("click")}
  {disabled}
>
  {#if icon}
    <Icon class="mt-[1px] w-5 h-5" {icon} />
  {/if}
  <slot />
</button>
