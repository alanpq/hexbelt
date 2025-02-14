<script lang="ts">
  import { cn } from "$lib/utils";
  import { onMount, onDestroy } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  //type $$Props = HTMLAttributes<HTMLTableRowElement> & {
  //  "data-state"?: unknown;
  //};

  export let rowAttrs;
  export let root = undefined;
  let className: HTMLAttributes<HTMLTableRowElement>["class"] = undefined;
  export { className as class };

  let observer: IntersectionObserver | undefined = undefined;

  let inView = false;
  let this_element: HTMLElement | undefined = undefined;

  onMount(() => {
    if (!this_element) return;
    observer = new IntersectionObserver(
      (e) => {
        inView = e[0].isIntersecting;
      },
      { root, rootMargin: "50% 0px 50% 0px" },
    );
    observer.observe(this_element);
  });
  onDestroy(() => {
    if (!this_element || !observer) return;
    observer.unobserve(this_element);
    observer.disconnect();
  });
</script>

<tr
  class={cn(
    "hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors",
    className,
  )}
  bind:this={this_element}
  {...rowAttrs}
  {...$$restProps}
  on:click
  on:keydown
>
  <slot {inView} />
</tr>
