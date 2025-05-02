<script lang="ts">
  import {
    type BinObject,
    type BinProperty,
    type BinEntryValue,
  } from "$lib/pkg/rust";
  import { get_type } from ".";
  interface Props {
    value: BinEntryValue & { kind: "PropertyMapEntry" };
  }

  let { value }: Props = $props();

  const SvelteComponent = $derived(get_type(value.value.value.kind));
</script>

<!-- {JSON.stringify(value.value)} -->
<SvelteComponent
  value={value.value.value}
/>
<!-- <span class="text-muted-foreground">{value.value}</span> -->
