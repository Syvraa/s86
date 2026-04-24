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
  let hertz = $state(2);
  let memSize = $state(64);
  let intervalId = $state(0);

  function createSimulator(src: string): Simulator {
    return new Simulator(src, memSize);
  }

  function run(hertz: number) {
    if (!simulator) {
      try {
        simulator = createSimulator(editor.getContent());
      } catch (err) {
        const error = err as SyntaxError;
        console.log(error);
        return;
      }
    }

    intervalId = setInterval(() => {
      try {
        editor.highlightLine(simulator?.current_line());
        stepSimulator();
      } catch (err) {
        const error = err as SimulatorError;
        if (error === SimulatorError.InvalidMemAccess) {
          console.log("Invalid memory access");
        }

        clearInterval(intervalId);
      }
    }, 1000 / hertz);
  }

  function stepSimulator() {
    if (simulator) {
      diff = simulator.step();
    }
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
      stepSimulator();
    } catch (err) {
      const error = err as SimulatorError;
      console.log(error);
    }
  }

  function reset() {
    simulator = null;
    if (intervalId) clearInterval(intervalId);
    registers.reset();
    memory.reset();
    editor.reset();
  }
</script>

<div id="container">
  <Editor bind:this={editor} />
  <div id="controls">
    <label>
      Memory size (bytes)
      <input type="text" bind:value={memSize} />
    </label>
    <br />
    <label>
      Run speed (Hz)
      <input type="text" bind:value={hertz} />
    </label>
    <br />
    <button
      onclick={() => {
        run(hertz);
      }}>Run</button
    >
    <button
      onclick={() => {
        if (intervalId) clearInterval(intervalId);
      }}>Stop</button
    >
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
