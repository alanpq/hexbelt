<script lang="ts">
  import { run } from 'svelte/legacy';

  import {
    createTable,
    Render,
    Subscribe,
    createRender,
  } from "svelte-headless-table";
  import {
    addSortBy,
    addExpandedRows,
    addSubRows,
  } from "svelte-headless-table/plugins";
  import {
    derived,
    readable,
    type Readable,
    type Writable,
  } from "svelte/store";
  import * as Table from "$lib/components/ui/table";

  import {
    Bin,
    type BinObject,
    type BinProperty,
    type BinEntry,
  } from "$lib/pkg/rust";
  import Icon from "@iconify/svelte";
  import { cn } from "$lib/utils";
  import { Button } from "$lib/components/ui/button";
  import ExpandIndicator from "./ExpandIndicator.svelte";
  import { types } from "./types";
  import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";

  interface Props {
    bin: Bin;
    data: Readable<BinEntry[]>;
  }

  let { bin, data }: Props = $props();

  const table = createTable(data, {
    sort: addSortBy({
      disableMultiSort: true,
    }),
    sub: addSubRows({
      children: "children",
    }),
    expand: addExpandedRows({}),
  });

  const columns = table.createColumns([
    table.column({
      accessor: (value) => value,
      id: "name",
      header: "Name",
      cell: ({ row, value }, { pluginStates }) => {
        const { isExpanded, canExpand, isAllSubRowsExpanded } =
          pluginStates.expand.getRowState(row);
        return createRender(ExpandIndicator, {
          value,
          depth: row.depth,
          isExpanded,
          canExpand,
          isAllSubRowsExpanded,
        });
      },
    }),
    table.column({
      id: "value",
      accessor: ({ value }) => value,
      header: "Value",

      cell: ({ row, value }, { pluginStates }) => {
        return createRender(types[value.kind], { value });
      },
    }),
  ]);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);

  const { sortKeys } = pluginStates.sort;
  const { expandedIds } = pluginStates.expand;
  run(() => {
    $sortKeys;
  });
</script>

<div class="rounded-md w-full h-full overflow-x-clip">
  <ScrollArea class="w-full h-full">
    <Table.Root {...$tableAttrs} class="w-full overflow-x-clip mb-14">
      <Table.Header>
        {#each $headerRows as headerRow}
          <Subscribe rowAttrs={headerRow.attrs()}>
            <Table.Row>
              {#each headerRow.cells as cell (cell.id)}
                <Subscribe
                  attrs={cell.attrs()}
                  
                  props={cell.props()}
                  
                >
                  {#snippet children({ attrs, props })}
                                    <Table.Head
                      {...attrs}
                      class={cn(
                        "py-0 px-1 h-7 text-ellipsis whitespace-nowrap overflow-hidden",
                        cell.id == "name" && "w-[20ch] pl-7",
                        cell.id == "expanded" ? "w-0" : null,
                      )}
                    >
                      <Render of={cell.render()} />
                    </Table.Head>
                                                    {/snippet}
                                </Subscribe>
              {/each}
            </Table.Row>
          </Subscribe>
        {/each}
      </Table.Header>
      <Table.Body {...$tableBodyAttrs}>
        {#each $pageRows as row (row.id)}
          <Subscribe rowAttrs={row.attrs()} >
            {#snippet children({ rowAttrs })}
                        <Table.Row {...rowAttrs}>
                {#each row.cells as cell (cell.id)}
                  {@const parent = row.parentRow}
                  <Subscribe
                    attrs={cell.attrs()}
                    
                    props={cell.props()}
                    
                  >
                    {#snippet children({ attrs, props })}
                                    <Table.Cell
                        {...attrs}
                        class={cn(
                          "py-0 px-1 h-7 text-ellipsis whitespace-nowrap overflow-hidden max-w-[50dvw] relative",
                        )}
                        style={`
                        transform: translateX(${row.depth}rem);
                        ${
                          cell.id == "value" ? `padding-right: ${row.depth}rem` : ""
                        }
                      `}
                      >
                        {#if cell.id === "name"}
                          {#if !(cell.isData() && !cell.value)}
                            <div class={cn("font-medium")}>
                              <Render of={cell.render()} />
                            </div>
                          {/if}
                        {:else}
                          <Render of={cell.render()} />
                        {/if}
                      </Table.Cell>
                                                      {/snippet}
                                </Subscribe>
                {/each}
              </Table.Row>
                                  {/snippet}
                    </Subscribe>
        {/each}
      </Table.Body>
    </Table.Root>
  </ScrollArea>
</div>
