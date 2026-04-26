<script lang="ts">
  import { StateDiff } from "s86-lib";
  import { MediaQuery } from "svelte/reactivity";

  let { diff }: { diff: StateDiff } = $props();

  let memory: number[] = $state([]);
  let bytesPerRow = $derived(new MediaQuery("min-width: 1280px").current ? 32 : 16);

  $effect(() => {
    for (const mem_diff of diff.mem_diffs) {
      const address = mem_diff.address;
      memory[address] = mem_diff.value;
    }
  });

  export function resize(size: number) {
    memory = new Array(size).fill(0);
  }

  export function reset() {
    memory.fill(0);
  }
</script>

<div
  class="memory"
  style:grid-template-rows="repeat({Math.ceil(memory.length / bytesPerRow) + 1}, 1em)"
  style:grid-template-columns="auto repeat({bytesPerRow}, 1fr) auto"
>
  <div class="header" style:grid-column="1">Addr</div>
  <div class="header" style:grid-column="2 / span {bytesPerRow}">Data</div>
  <div class="header" style:grid-column={bytesPerRow + 2}>Addr</div>
  {#each memory as byte, i (i)}
    {@const row = Math.floor(i / bytesPerRow) + 2}
    {@const col = (i % bytesPerRow) + 2}
    {#if i % bytesPerRow === 0}
      <div class="address" style:grid-row={row} style:grid-column={1}>0x{i.toString(16)}</div>
    {/if}
    <div
      class="byte"
      class:highlight={diff.mem_diffs.some((d) => d.address == i)}
      style:grid-row={row}
      style:grid-column={col}
    >
      {byte.toString(16)}
    </div>
    {#if i % bytesPerRow === bytesPerRow - 1}
      <div class="address" style:grid-row={row} style:grid-column={bytesPerRow + 2}>
        0x{i.toString(16)}
      </div>
    {/if}
  {/each}
</div>

<style>
  .memory {
    width: 100%;
    display: grid;
    flex: 1;
    overflow-y: scroll;
    font-family: monospace;
    text-align: center;
  }

  .highlight {
    background-color: blueviolet;
  }

  .header {
    grid-row: 1;
    position: sticky;
    top: 0;
    background-color: white;
  }

  .byte {
    min-width: 3ch;
  }
</style>
