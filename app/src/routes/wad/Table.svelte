<script lang="ts">
  import {
    createTable,
    Render,
    Subscribe,
    createRender,
  } from "svelte-headless-table";
  import { addSortBy } from "svelte-headless-table/plugins";
  import {
    derived,
    readable,
    type Readable,
    type Writable,
  } from "svelte/store";
  import * as Table from "$lib/components/ui/table";

  import { Item, WadTree } from "rust";
  import TableActions from "./TableActions.svelte";
  import Icon from "@iconify/svelte";
  import { cn } from "$lib/utils";
  import { Button } from "$lib/components/ui/button";

  export let wad: WadTree;
  export let data: Readable<Item[]>;
  export let path: Writable<number[]>;

  const table = createTable(data, {
    sort: addSortBy({
      disableMultiSort: true,
    }),
  });

  function humanFileSize(size: number) {
    var i = size == 0 ? 0 : Math.floor(Math.log(size) / Math.log(1024));
    return (
      Number((size / Math.pow(1024, i)).toFixed(2)) +
      " " +
      ["B", "kB", "MB", "GB", "TB"][i]
    );
  }

  const ext_icons: Record<string, string> = {
    anm: "material-symbols:animation",
    bin: "mdi:file",
    bnk: "mdi:file",
    dds: "mdi:image",
    jpg: "mdi:image",
    luaobj: "mdi:file",
    mapgeo: "mdi:file",
    png: "mdi:image",
    preload: "mdi:file",
    scb: "mdi:cube-outline",
    sco: "mdi:cube-outline",
    skl: "mdi:skull",
    skn: "ion:body",
    stringtable: "mdi:text",
    svg: "ph:file-svg",
    tex: "mdi:image",
    wgeo: "mdi:file",
    wpk: "mdi:file",
  };

  const sort_icons = {
    asc: "mdi:sort-ascending",
    desc: "mdi:sort-descending",
  } as const;

  const columns = table.createColumns([
    table.column({
      accessor: ({ children, name }) => ({
        children: children?.length ?? 0,
        ext: name.split(".").at(-1),
      }),
      header: "",
      id: "icon",
      cell: ({ value: { children, ext } }) =>
        createRender(Icon, {
          icon:
            (ext && ext_icons[ext]) ||
            (children > 0 ? "mdi:folder" : "mdi:file"),
          class: "w-4 h-4",
        }),
      plugins: {
        sort: {
          disable: true,
        },
      },
    }),

    table.column({
      accessor: "name",
      header: "Name",
    }),
    table.column({
      accessor: "size",
      header: "Size",
      cell: ({ value }) => humanFileSize(value),
    }),
    table.column({
      accessor: (item) => item,
      id: "actions",
      header: "",
      cell: ({ value }) => {
        return createRender(TableActions, { item: value, wad });
      },
      plugins: {
        sort: {
          disable: true,
        },
      },
    }),
  ]);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);

  const { sortKeys } = pluginStates.sort;
  $: $sortKeys;
</script>

<div class="rounded-md w-full">
  <Table.Root {...$tableAttrs} class="w-full">
    <Table.Header>
      {#each $headerRows as headerRow}
        <Subscribe rowAttrs={headerRow.attrs()}>
          <Table.Row>
            {#each headerRow.cells as cell (cell.id)}
              <Subscribe
                attrs={cell.attrs()}
                let:attrs
                props={cell.props()}
                let:props
              >
                <Table.Head
                  {...attrs}
                  class={cn(
                    "py-0 px-1 h-7 text-ellipsis whitespace-nowrap overflow-hidden max-w-[50dvw]",
                    cell.id == "icon" || cell.id == "actions" ? "w-0" : null,
                  )}
                >
                  {#if cell.id === "size"}
                    {@const order = $sortKeys.find((v, i) => v.id == "size")}
                    <div class="flex justify-end">
                      <Button
                        variant="ghost"
                        class=""
                        on:click={props.sort.toggle}
                      >
                        <Render of={cell.render()} />
                        {#if order}
                          <Icon
                            icon={sort_icons[order.order]}
                            class="ml-2 h-4 w-4"
                          />
                        {/if}
                      </Button>
                    </div>
                  {:else}
                    <Render of={cell.render()} />
                  {/if}
                </Table.Head>
              </Subscribe>
            {/each}
          </Table.Row>
        </Subscribe>
      {/each}
    </Table.Header>
    <Table.Body {...$tableBodyAttrs}>
      <Table.Row
        class={cn(
          "cursor-pointer",
          $path.length == 0 && "cursor-default text-muted-foreground",
        )}
        on:click={() => {
          if ($path.length == 0) return;
          $path.pop();
          $path = $path;
        }}
      >
        <Table.Cell colspan={1} class="py-0 px-1 h-7"
          ><Icon icon="mdi:folder-arrow-up" class="h-4 w-4" /></Table.Cell
        >
        <Table.Cell colspan={3} class="py-0 px-1 h-7">..</Table.Cell>
      </Table.Row>
      {#each $pageRows as row (row.id)}
        <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
          <Table.Row
            {...rowAttrs}
            class={cn(
              row.isData() && row.original.is_dir() && "cursor-pointer",
            )}
            on:click={() => {
              if (row.isData() && row.original.is_dir()) {
                $path = [...$path, row.original.id];
              }
            }}
          >
            {#each row.cells as cell (cell.id)}
              <Subscribe attrs={cell.attrs()} let:attrs>
                <Table.Cell
                  {...attrs}
                  class="py-0 px-1 h-7 text-ellipsis whitespace-nowrap overflow-hidden max-w-[50dvw]"
                >
                  {#if cell.id === "size"}
                    <div class="text-right font-medium">
                      <Render of={cell.render()} />
                    </div>
                  {:else}
                    <Render of={cell.render()} />
                  {/if}
                </Table.Cell>
              </Subscribe>
            {/each}
          </Table.Row>
        </Subscribe>
      {/each}
    </Table.Body>
  </Table.Root>
</div>
