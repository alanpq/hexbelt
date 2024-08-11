<script lang="ts">
  import * as stores from "$lib/stores";

  import { Bin, open_bin } from "rust";
  import { toast } from "svelte-sonner";

  import FilePicker from "$lib/components/FilePicker.svelte";

  let bin_src = stores.bin_src();

  $: bin = $bin_src && Bin.from_bytes($bin_src);
</script>

<header>
  <FilePicker
    class="flex-shrink w-min"
    on:open={async ({ detail: files }) => {
      try {
        bin = await open_bin(files[0]);
      } catch (e) {
        console.error(e);
        toast.error(`${e}`);
      }
    }}
  />
</header>
{#if bin}
  Hi bin!
{/if}
