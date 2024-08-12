<script lang="ts">
  import { type BinObject, type BinProperty, type BinEntryValue } from "rust";

  import { Input } from "$lib/components/ui/input";
  import Checkbox from "$lib/components/ui/checkbox/checkbox.svelte";

  export let value: BinEntryValue & { kind: "PropertyJSValue" };
  $: inner = value.value;
</script>

{#if inner.kind == "String"}
  <span class="bg-secondary px-1 py-0.5 h-full text-amber-500">
    "<span class="font-mono rounded-sm">{inner.value}</span>"
  </span>
{:else if ["U", "I", "F"].indexOf(inner.kind[0]) != -1}
  <span class="bg-secondary px-1 py-0.5 h-full text-cyan-300">
    <span class="font-mono rounded-sm">{inner.value}</span>
  </span>
  <span class="px-1 py-0.5 h-full text-cyan-300">
    <span class="font-mono rounded-sm">({inner.kind.toLowerCase()})</span>
  </span>
{:else if inner.kind == "Bool" || inner.kind == "BitBool"}
  <Checkbox disabled value={inner.value} />
{:else if inner.kind == "Hash"}
  <span class="bg-secondary px-1 py-0.5 h-full text-lime-200">
    <span class="font-mono rounded-sm">{inner.value}</span>
  </span>
{:else if inner.kind == "ObjectLink"}
  <span class="bg-secondary px-1 py-0.5 h-full text-lime-200">
    <span class="font-mono rounded-sm">{inner.value}</span>
  </span>
  <span class="px-1 py-0.5 h-full text-lime-200">
    <span class="font-mono rounded-sm">(object link)</span>
  </span>
{:else}
  Unknown js '{inner.kind}' - {JSON.stringify(inner.value)}
{/if}
