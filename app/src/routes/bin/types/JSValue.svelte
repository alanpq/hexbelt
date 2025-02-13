<script lang="ts">
  import {
    type BinObject,
    type BinProperty,
    type BinEntryValue,
  } from "$lib/pkg/rust";

  import { Input } from "$lib/components/ui/input";
  import Checkbox from "$lib/components/ui/checkbox/checkbox.svelte";
  import Toggle from "$lib/components/ui/toggle/toggle.svelte";
  import Icon from "@iconify/svelte";
  import * as Tooltip from "$lib/components/ui/tooltip";
  import { cn } from "$lib/utils";
  import Color from "./Color.svelte";

  export let value: BinEntryValue & { kind: "PropertyJSValue" };
  $: inner = value.value;

  const is_vec = (
    t: (typeof value)["value"],
  ): t is { kind: string; value: number[] } => {
    return t.kind.slice(0, 6) == "Vector";
  };

  const is_color = (t: number[]): t is [number, number, number, number] =>
    t.length == 4;

  const vec_to_color = (t: [number, number, number, number]) =>
    t.map((c) => c * 255) as [number, number, number, number];

  let as_color = false;
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
    <span class="font-mono rounded-sm">0x{inner.value.toString(16)}</span>
  </span>
{:else if inner.kind == "ObjectLink"}
  <span class="bg-secondary px-1 py-0.5 h-full text-lime-200">
    <span class="font-mono rounded-sm">0x{inner.value.toString(16)}</span>
  </span>
  <span class="px-1 py-0.5 h-full text-lime-200">
    <span class="font-mono rounded-sm">(object link)</span>
  </span>
{:else if inner.kind == "Color"}
  <Color color={inner.value} />
{:else if is_vec(inner)}
  <span
    class={cn(
      "grid gap-1 py-0.5 w-full h-full items-center grid-cols-[minmax(min-content,max-content),auto,min-content]",
      as_color && "grid-cols-[auto,min-content]",
    )}
  >
    {#if as_color && is_color(inner.value)}
      <Color color={vec_to_color(inner.value)} />
    {:else}
      <div
        class={"grid gap-1"}
        style={`grid-template-columns: repeat(${inner.value.length}, minmax(auto, 12ch))`}
      >
        {#each inner.value as i}
          <Tooltip.Root>
            <Tooltip.Trigger asChild let:builder>
              <div {...builder} use:builder.action>
                <Input
                  type="text"
                  disabled
                  class="bg-secondary px-1.5 text-ellipsis overflow-clip text-cyan-300 h-5 font-mono rounded-sm"
                  value={i.toString().length < 4 ? i.toPrecision(4) : i}
                />
              </div>
            </Tooltip.Trigger>
            <Tooltip.Content>{i}</Tooltip.Content>
          </Tooltip.Root>
        {/each}
      </div>
      <span class="px-1 py-0.5 h-full text-cyan-300">
        <span class="font-mono rounded-sm">({inner.kind.toLowerCase()})</span>
      </span>
    {/if}
    {#if is_color(inner.value)}
      <Tooltip.Root>
        <Tooltip.Trigger asChild let:builder>
          <div {...builder} use:builder.action>
            <Toggle class="h-6 aspect-square p-0 mr-5" bind:pressed={as_color}>
              <Icon icon="mdi:color" class="w-4 h-4 opacity-40" />
            </Toggle>
          </div>
        </Tooltip.Trigger>
        <Tooltip.Content>Interpret as colour</Tooltip.Content>
      </Tooltip.Root>
    {/if}
  </span>
{:else}
  Unknown js '{inner.kind}' - {JSON.stringify(inner.value)}
{/if}
