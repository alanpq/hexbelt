<script lang="ts">
  import {
    type BinObject,
    type BinProperty,
    type BinEntryValue,
  } from "$lib/pkg/rust";
  interface Props {
    value: BinEntryValue;
  }

  let { value }: Props = $props();
</script>

"{value.kind}"?
