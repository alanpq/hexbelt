<script lang="ts">
  import {
    type BinObject,
    type BinProperty,
    type BinEntryValue,
  } from "$lib/pkg/rust";
  import { get_type } from ".";
  interface Props {
    value: BinEntryValue & { kind: "PropertyOptional" };
  }

  let { value }: Props = $props();
</script>

<!-- <span class="text-muted-foreground">{value.kind}</span> -->
{#if value.value}
  {@const SvelteComponent = get_type(value.value.kind)}
  <SvelteComponent value={value.value} />
  <span class="text-muted-foreground">optional</span>
{:else}
  <span class="text-muted-foreground">null</span>
{/if}
