<script lang="ts">
  import { open_wad } from "rust";

  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";

  import { onMount } from "svelte";
  let input: HTMLInputElement | null = null;

  onMount(() => {
    input = document.getElementById("input") as any;
  });

  let wad;
</script>

<Input id="input" type="file" />
<Button
  on:click={async () => {
    if (!input || !input.files || input.files.length < 1) return;
    wad = await open_wad(input.files[0]);
    console.log(wad.children());
  }}
/>
