<script lang="ts">
  import "../app.css";
  import {ModeWatcher} from "mode-watcher";

  import {onMount} from "svelte";
  import init from "rust";
  onMount(async () => {
    await init();
  });
</script>

<ModeWatcher/>
<slot></slot>
