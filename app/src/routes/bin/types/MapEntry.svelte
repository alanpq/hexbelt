<script lang="ts">
  import {
    type BinObject,
    type BinProperty,
    type BinEntryValue,
  } from "$lib/pkg/rust";
  import { get_type } from ".";
  export let value: BinEntryValue & { kind: "PropertyMapEntry" };
</script>

<!-- {JSON.stringify(value.value)} -->
<svelte:component
  this={get_type(value.value.value.kind)}
  value={value.value.value}
/>
<!-- <span class="text-muted-foreground">{value.value}</span> -->
