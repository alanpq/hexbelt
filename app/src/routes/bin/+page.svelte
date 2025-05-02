<script lang="ts">
  import { run } from 'svelte/legacy';

  import * as stores from "$lib/stores";

  import { Bin, open_bin, type BinEntry, type TreeNode } from "$lib/pkg/rust";
  import { toast } from "svelte-sonner";

  import FilePicker from "$lib/components/FilePicker.svelte";
  import Table from "./Table.svelte";
  import { writable } from "svelte/store";

  let loaded = stores.bin_hashtables();

  let bin = stores.bin();

  let data = writable<BinEntry[]>([]);

  const make_entry = (bin: Bin, node: TreeNode): BinEntry => {
    if (node.kind == "Namespace") {
      return {
        name: node.value[0],
        value: { kind: "Namespace" },
        children: Object.entries(node.value[1])
          .sort((a, b) => {
            return a[0].localeCompare(b[0]);
          })
          .map(([_, n]) => {
            return make_entry(bin, n);
          }),
      };
    }
    const entry = bin.data.objects[node.value[1]];
    return entry;
  };

  run(() => {
    $bin && data.set(make_entry($bin, $bin.data.tree).children);
  });
</script>

<header>
  <FilePicker
    class="flex-shrink w-min"
    disabled={!$loaded}
    on:open={async ({ detail: files }) => {
      try {
        $bin = await open_bin(files[0]);
        console.log({ bin });
      } catch (e) {
        console.error(e);
        toast.error(`${e}`);
      }
    }}>Open Bin</FilePicker
  >
</header>
{#if $bin}
  <Table bin={$bin} {data} />
{/if}
