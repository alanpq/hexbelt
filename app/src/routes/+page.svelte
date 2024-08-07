<script lang="ts">
  import { open_wad, load_hashtables, WadTree } from "rust";

  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";

  import { onMount } from "svelte";
  import { writable, type Writable } from "svelte/store";
  let input: HTMLInputElement | null = null;

  onMount(async () => {
    input = document.getElementById("input") as any;
  });

  let wad: Writable<WadTree | null> = writable(null);
  let path: number[] = [];
</script>

<Input id="input" type="file" />
<Button
  on:click={async () => {
    if (!input || !input.files || input.files.length < 1) return;
    try {
      $wad = await open_wad(input.files[0]);
      console.log(Array.from($wad.children).map((i) => $wad?.get(i)));
    } catch (e) {
      console.error("failed to open wad: ", e);
    }
  }}
/>

{#if $wad}
  {@const view =
    (path.length == 0
      ? $wad.children
      : $wad.get(path[path.length - 1])?.children) ?? []}
  <ul>
    {#if path.length > 0}
      <li>
        <button
          class="w-full p-2 text-left hover:bg-white/10"
          on:click={() => {
            path.pop();
            path = path;
          }}
          >..
        </button>
      </li>
    {/if}
    {#each view as i}
      {@const child = $wad.get(i)}
      <li>
        <button
          class="w-full p-2 text-left hover:bg-white/10"
          on:click={() => {
            if (child?.is_dir()) {
              path.push(i);
              path = path;
            }
          }}
        >
          {child?.name}
        </button>
      </li>
    {/each}
  </ul>
{/if}
