<script lang="ts">
  import { StateDiff } from "s86-lib";

  let { diff, memSize }: { diff: StateDiff; memSize: number } = $props();

  // We use $state + $effect, because we want to be able to update the size of the memory later.
  // eslint-disable-next-line svelte/prefer-writable-derived
  let memory: number[] = $state([]);

  $effect(() => {
    memory = new Array(Number(memSize)).fill(0);
  });

  $effect(() => {
    for (const mem_diff of diff.mem_diffs) {
      const address = mem_diff.address;
      memory[address] = mem_diff.value;
    }
  });

  export function reset() {
    memory.fill(0);
  }
</script>

<div id="memory">
  {#each memory as byte, i (i)}
    <div class="byte">{byte}</div>
  {/each}
</div>

<style>
  #memory {
    width: 100%;
    overflow-y: scroll;
  }

  .byte {
    width: 3ch;
    display: inline-block;
  }
</style>
