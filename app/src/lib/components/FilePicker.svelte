<script lang="ts">
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { createEventDispatcher } from "svelte";
  import { cn } from "$lib/utils";

  export let disabled: boolean = false;

  const dispatch = createEventDispatcher<{
    open: FileList;
  }>();

  let files: FileList | null;

  let className: string | null = null;
  export { className as class };
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
    <slot />
  </Button>
</section>
