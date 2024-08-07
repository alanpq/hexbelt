<script lang="ts">
  import "../app.css";
  import { ModeWatcher } from "mode-watcher";

  import { onMount, setContext } from "svelte";
  import init, { load_hashtables } from "rust";
  import { writable } from "svelte/store";

  let hashtables = writable(false);
  setContext("hashtables", hashtables);

  onMount(async () => {
    await init();
    load_hashtables().then(() => hashtables.set(true));
  });
</script>

<ModeWatcher />
<slot></slot>
