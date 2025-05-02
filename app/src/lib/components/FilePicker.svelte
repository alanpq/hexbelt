<script lang="ts">
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { createEventDispatcher } from "svelte";
  import { cn } from "$lib/utils";


  const dispatch = createEventDispatcher<{
    open: FileList;
  }>();

  let files: FileList | null = $state();

  interface Props {
    disabled?: boolean;
    class?: string | null;
    children?: import('svelte').Snippet;
  }

  let { disabled = false, class: className = null, children }: Props = $props();
  
</script>

<section class="flex">
  <input
    type="file"
    bind:files
    class={cn(
      "border-input placeholder:text-muted-foreground focus-visible:ring-ring flex h-9 w-full rounded-md border bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50",
      className,
    )}
  />
  <Button
    {disabled}
    on:click={async () => {
      if (!files || files.length == 0) return;
      dispatch("open", files);
    }}
  >
    {@render children?.()}
  </Button>
</section>
