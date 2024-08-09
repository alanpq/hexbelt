<script lang="ts">
  import "../app.css";
  import { onMount, setContext } from "svelte";
  import init, { load_hashtables } from "rust";
  import { writable } from "svelte/store";
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

  let hashtables = writable(false);
  setContext("hashtables", hashtables);

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
    toast.info("Loading hashtables...", {
      id: "hashtables",
    });
    load_hashtables(`${base}/hashes`).then((entries) => {
      if (entries <= 0) {
        toast.error(`Hashtables were empty! Could not load any hashes.`, {
          id: "hashtables",
        });
      } else if (entries <= 1000) {
        toast.warning(`Loaded ${entries.toLocaleString()} hashes?`, {
          id: "hashtables",
        });
      } else {
        toast.success(`Loaded ${entries.toLocaleString()} hashes!`, {
          id: "hashtables",
        });
      }
      hashtables.set(true);
    });
  });
</script>

<ModeWatcher />
<Toaster richColors />

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
