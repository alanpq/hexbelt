<script lang="ts">
  import {
    type BinObject,
    type BinProperty,
    type BinEntryValue,
  } from "$lib/pkg/rust";
  import { get_type } from ".";
  export let value: BinEntryValue & { kind: "PropertyOptional" };
</script>

<!-- <span class="text-muted-foreground">{value.kind}</span> -->
{#if value.value}
  <svelte:component this={get_type(value.value.kind)} value={value.value} />
  <span class="text-muted-foreground">optional</span>
{:else}
  <span class="text-muted-foreground">null</span>
{/if}
