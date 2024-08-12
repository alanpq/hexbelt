<script lang="ts">
  import { type BinObject, type BinProperty, type BinEntryValue } from "rust";
  export let value: BinEntryValue & { kind: "Object" };
</script>

<span class="text-muted-foreground">{value.kind.slice(8)}</span>
