<script lang="ts">
  import { createBubbler } from 'svelte/legacy';

  const bubble = createBubbler();
  import { cn } from "$lib/utils";
  import { onMount, onDestroy } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  //type $$Props = HTMLAttributes<HTMLTableRowElement> & {
  //  "data-state"?: unknown;
  

  interface Props {
    //};
    rowAttrs: any;
    root?: any;
    class?: HTMLAttributes<HTMLTableRowElement>["class"];
    children?: import('svelte').Snippet<[any]>;
    [key: string]: any
  }

  let {
    rowAttrs,
    root = undefined,
    class: className = undefined,
    children,
    ...rest
  }: Props = $props();
  

  let observer: IntersectionObserver | undefined = undefined;

  let inView = $state(false);
  let this_element: HTMLElement | undefined = $state(undefined);

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
  {...rest}
  onclick={bubble('click')}
  onkeydown={bubble('keydown')}
>
  {@render children?.({ inView, })}
</tr>
