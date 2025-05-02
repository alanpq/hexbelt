<script lang="ts">
  import Icon from "@iconify/svelte";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{
    click: void;
  }>();
  interface Props {
    icon?: string | undefined;
    disabled?: boolean;
    children?: import('svelte').Snippet;
  }

  let { icon = undefined, disabled = false, children }: Props = $props();
</script>

<button
  class="w-full text-left px-2 hover:bg-white/10 flex items-center gap-1 rounded-md disabled:text-muted-foreground disabled:hover:bg-transparent"
  onclick={() => dispatch("click")}
  {disabled}
>
  {#if icon}
    <Icon class="my-2 w-5 h-5" {icon} />
  {/if}
  {@render children?.()}
</button>
