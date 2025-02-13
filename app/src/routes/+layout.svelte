<script lang="ts">
  import "../app.css";
  import { onMount, setContext } from "svelte";
  import init, {
    load_wad_hashtables,
    load_bin_hashtables,
  } from "$lib/pkg/rust";
  import { writable, type Writable } from "svelte/store";
  import Nav from "./nav.svelte";

  import { browser } from "$app/environment";
  import { base } from "$app/paths";

  import { routes } from "$lib/config";

  import { ModeWatcher } from "mode-watcher";

  import * as Resizable from "$lib/components/ui/resizable";
  import { Toaster } from "$lib/components/ui/sonner";
  import { toast } from "svelte-sonner";
  import { cn } from "$lib/utils";

  let defaultLayout = (browser
    ? JSON.parse(localStorage.getItem("layout") || "0")
    : null) || [265, 440];
  let navCollapsedSize = 10;

  setContext("wad", writable(null));
  setContext("wad_path", writable([]));
  setContext("bin", writable(null));

  let wad_hashtables = writable(false);
  setContext("wad_hashtables", wad_hashtables);
  let bin_hashtables = writable(false);
  setContext("bin_hashtables", bin_hashtables);

  let isCollapsed = browser ? !!localStorage.getItem("collapsed") : false;

  const onLayoutChange = (sizes: number[]) => {
    if (!browser) return;
    localStorage.setItem("layout", JSON.stringify(sizes));
  };
  const onCollapse = () => {
    isCollapsed = true;
    if (browser) localStorage.setItem("collapsed", "1");
  };
  const onExpand = () => {
    isCollapsed = false;
    if (browser) localStorage.removeItem("collapsed");
  };

  onMount(async () => {
    try {
      await init();
    } catch (e) {
      toast.error("Failed to load WASM module!", {
        important: true,
        description: `${e}`,
      });
      console.error("Failed to load WASM module:", e);
    }

    const load_hashes = (
      func: typeof load_bin_hashtables,
      name: string,
      store: Writable<boolean>,
    ) => {
      toast.info(`Loading ${name} hashtables...`, {});
      func(`${base}/hashes`).then((count) => {
        if (count <= 0) {
          toast.error(
            `${name} Hashtables were empty! Could not load any hashes.`,
          );
        } else if (count <= 1000) {
          toast.warning(`Loaded ${count.toLocaleString()} ${name} hashes?`, {});
        } else {
          toast.success(`Loaded ${count.toLocaleString()} ${name} hashes!`, {});
        }
        store.set(true);
      });
    };
    load_hashes(load_wad_hashtables, "Wad", wad_hashtables);
    load_hashes(load_bin_hashtables, "Bin", bin_hashtables);
  });
</script>

<ModeWatcher />
<Toaster richColors />
<div id="color-portal" />

<Resizable.PaneGroup
  direction="horizontal"
  {onLayoutChange}
  class="h-full w-full items-stretch"
>
  <Resizable.Pane
    class={cn("min-w-min", isCollapsed && "max-w-max")}
    defaultSize={defaultLayout[0]}
    collapsedSize={navCollapsedSize}
    collapsible
    minSize={5}
    maxSize={30}
    {onCollapse}
    {onExpand}
  >
    <Nav {routes} {isCollapsed} />
  </Resizable.Pane>
  <Resizable.Handle withHandle />
  <Resizable.Pane defaultSize={defaultLayout[1]} minSize={30}>
    <slot></slot>
  </Resizable.Pane>
</Resizable.PaneGroup>

<style lang="postcss">
  :global(html, body) {
    height: 100%;
  }
</style>
