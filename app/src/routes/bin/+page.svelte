<script lang="ts">
  import * as stores from "$lib/stores";

  import { Bin, open_bin, type BinEntry } from "rust";
  import { toast } from "svelte-sonner";

  import FilePicker from "$lib/components/FilePicker.svelte";
  import Table from "./Table.svelte";
  import { writable } from "svelte/store";

  let loaded = stores.bin_hashtables();

  let bin_src = stores.bin_src();

  let data = writable<BinEntry[]>([]);

  $: bin = $bin_src && Bin.from_bytes($bin_src);
  $: bin && data.set(bin.tree.objects);
</script>

<header>
  <FilePicker
    class="flex-shrink w-min"
    disabled={!$loaded}
    on:open={async ({ detail: files }) => {
      try {
        bin = await open_bin(files[0]);
        console.log({ bin });
      } catch (e) {
        console.error(e);
        toast.error(`${e}`);
      }
    }}>Open Bin</FilePicker
  >
</header>
{#if bin}
  <Table {bin} {data} />
{/if}
