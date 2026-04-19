<script lang="ts">
  import { Simulator, SimulatorError, StateDiff, SyntaxError } from "s86-lib";
  import Registers from "./lib/Registers.svelte";
  import Editor from "./lib/Editor.svelte";

  let diff = $state(StateDiff.default());
  let simulator: Simulator | null = $state(null);
  let registers: Registers;
  let editor: Editor;

  function createSimulator(src: string): Simulator {
    return new Simulator(src, 64);
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
    editor.reset();
  }
</script>

<div id="container">
  <Editor bind:this={editor} />
  <div id="controls">
    <button onclick={step}>Step</button>
    <button onclick={reset}>Reset</button>
    <br />
    <Registers bind:this={registers} {diff} />
  </div>
</div>

<style>
  :global(body) {
    margin: 0px;
    height: 100vh;
  }

  :global(#app) {
    width: 100%;
    height: 100%;
  }

  #container {
    position: relative;
    width: 100%;
    height: 100%;
  }

  #controls {
    position: absolute;
    left: 50%;
  }
</style>
