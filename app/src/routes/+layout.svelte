<script lang="ts">
  import "../app.css";
  import { ModeWatcher } from "mode-watcher";

  import { onMount } from "svelte";
  import init, { load_hashtables } from "rust";
  onMount(async () => {
    await init();
    await load_hashtables();
  });
</script>

<ModeWatcher />
<slot></slot>
