<script lang="ts">
  let { name, value, highlight }: { name: string; value: bigint; highlight: boolean } = $props();
</script>

<div class="container" class:highlight>
  <div class="name">{name}</div>
  <div class="value">{value}</div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .highlight {
    background-color: blueviolet;
  }
</style>
