<script lang="ts">
  import { Simulator, SimulatorError, StateDiff, SyntaxError } from "s86-lib";
  import Registers from "./lib/Registers.svelte";
  import Editor from "./lib/Editor.svelte";
  import Memory from "./lib/Memory.svelte";

  let diff = $state(StateDiff.default());
  let simulator: Simulator | null = $state(null);
  let registers: Registers;
  let memory: Memory;
  let editor: Editor;

  let memSize = $state(64);

  function createSimulator(src: string): Simulator {
    return new Simulator(src, memSize);
  }

  function step() {
    if (!simulator) {
      try {
        simulator = createSimulator(editor.getContent());
      } catch (err) {
        const error = err as SyntaxError;
        console.log(error);
        return;
      }
    }

    editor.highlightLine(simulator.current_line());
    try {
      diff = simulator.step();
    } catch (err) {
      const error = err as SimulatorError;
      console.log(error);
    }
  }

  function reset() {
    simulator = null;
    registers.reset();
    memory.reset();
    editor.reset();
  }
</script>

<div id="container">
  <Editor bind:this={editor} />
  <div id="controls">
    <input type="text" name="memsize" id="memsize" bind:value={memSize} />
    <button onclick={step}>Step</button>
    <button onclick={reset}>Reset</button>
    <br />
    <Registers bind:this={registers} {diff} />
    <br />
    <Memory bind:this={memory} {diff} {memSize} />
  </div>
</div>

<style>
  #container {
    position: relative;
    width: 100%;
    height: 100%;
  }

  #controls {
    position: absolute;
    left: 50%;
    font-size: large;
    button {
      font-size: medium;
    }
  }
</style>
