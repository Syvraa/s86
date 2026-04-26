<script lang="ts">
  import { StateDiff } from "s86-lib";

  let { diff }: { diff: StateDiff } = $props();

  let memory: number[] = $state([]);

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

<div class="memory" style:grid-template-rows="repeat({Math.ceil(memory.length / 32) + 1}, 1em)">
  <div class="header" style:grid-column="1">Addr</div>
  <div class="header" style:grid-column="2 / span 32">Data</div>
  <div class="header" style:grid-column="34">Addr</div>
  {#each memory as byte, i (i)}
    {@const row = Math.floor(i / 32) + 2}
    {@const col = (i % 32) + 2}
    {#if i % 32 === 0}
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
    {#if i % 32 === 31}
      <div class="address" style:grid-row={row} style:grid-column={34}>0x{i.toString(16)}</div>
    {/if}
  {/each}
</div>

<style>
  .memory {
    width: 100%;
    display: grid;
    grid-template-columns: auto repeat(32, 1fr) auto;
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
