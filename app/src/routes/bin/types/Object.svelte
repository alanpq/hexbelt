<script lang="ts">
  import {
    type BinObject,
    type BinProperty,
    type BinEntryValue,
  } from "$lib/pkg/rust";
  interface Props {
    value: BinEntryValue & { kind: "Object" };
  }

  let { value }: Props = $props();
</script>

<span class="text-muted-foreground">{value.kind}</span>
