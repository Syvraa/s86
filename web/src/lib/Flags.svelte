<script lang="ts">
  let { value, highlight }: { value: bigint; highlight: boolean } = $props();
</script>

<div class="container" class:highlight>
  <div class="name">FLAGS</div>
  <div class="values">
    <span>[</span>
    {#if (value & 0x1n) != 0n}
      <span>CF</span>
    {/if}
    {#if (value & 0x40n) != 0n}
      <span>ZF</span>
    {/if}
    {#if (value & 0x80n) != 0n}
      <span>SF</span>
    {/if}
    {#if (value & 0x800n) != 0n}
      <span>OF</span>
    {/if}
    <span>]</span>
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .values {
    display: flex;
    width: 100%;
    max-width: 15ch;
    justify-content: center;
    text-align: center;

    span {
      margin: 0px 1ch;
    }
  }

  .highlight {
    background-color: blueviolet;
  }
</style>
