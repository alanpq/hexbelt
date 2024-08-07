<script lang="ts">
  import "../app.css";
  import { ModeWatcher } from "mode-watcher";

  import { onMount, setContext } from "svelte";
  import init, { load_hashtables } from "rust";
  import { writable } from "svelte/store";
  import Nav from "./nav.svelte";

  import { browser } from "$app/environment";

  import { routes } from "$lib/config";

  import * as Resizable from "$lib/components/ui/resizable";

  let defaultLayout = (browser
    ? JSON.parse(localStorage.getItem("layout") || "0")
    : null) || [265, 440];
  let navCollapsedSize = 5;

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
    await init();
    load_hashtables().then(() => hashtables.set(true));
  });
</script>

<ModeWatcher />

<Resizable.PaneGroup
  direction="horizontal"
  {onLayoutChange}
  class="h-full w-full items-stretch"
>
  <Resizable.Pane
    defaultSize={defaultLayout[0]}
    collapsedSize={navCollapsedSize}
    collapsible
    minSize={15}
    maxSize={50}
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
