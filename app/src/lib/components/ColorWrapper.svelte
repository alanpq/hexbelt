<script lang="ts">
  import { portal } from "svelte-portal";

  let {
    wrapper = $bindable(),
    isOpen,
    isDialog,
    children
  } = $props();
</script>

<div
  use:portal={"#color-portal"}
  bind:this={wrapper}
  class="wrapper z-[100] bg-secondary text-primary-foreground"
  class:is-open={isOpen}
  role={isDialog ? "dialog" : undefined}
  aria-label="color picker"
>
  {@render children?.()}
</div>

<style>
  div {
    padding: 8px;
    /* background-color: var(--cp-bg-color, black); */
    margin: 0 10px 10px;
    border-radius: 12px;
    display: none;
    width: max-content;
  }
  .is-open {
    display: inline-block;
  }
  [role="dialog"] {
    position: absolute;
    top: calc(var(--input-size, 25px) + 36px);
    left: 8px;
    z-index: 500;
  }
</style>
