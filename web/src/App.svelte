<script lang="ts">
  import {
    DiffReg,
    MemDiff,
    RegDiff,
    Simulator,
    SimulatorError,
    StateDiff,
    SyntaxError,
  } from "s86-lib";
  import Registers from "./lib/Registers.svelte";
  import Editor from "./lib/Editor.svelte";
  import Memory from "./lib/Memory.svelte";
  import { onMount } from "svelte";

  let diff = $state(StateDiff.default());
  let simulator: Simulator | null = $state(null);
  let registers: Registers;
  let memory: Memory;
  let editor: Editor;
  let hertz = $state(2);
  let memSize = $state(64);
  let intervalId = $state(0);
  let memoryError = $state("");

  function createSimulator() {
    if (!memSize) return;
    try {
      simulator = new Simulator(editor.getContent(), memSize);
      diff = {
        reg_diffs: [{ reg: DiffReg.Rsp, value: BigInt(memSize - 1) } as RegDiff],
        mem_diffs: [] as MemDiff[],
      } as StateDiff;
    } catch (err) {
      const error = err as SyntaxError;
      console.log(error);
      return;
    }
  }

  function stepSimulator() {
    if (simulator) {
      diff = simulator.step();
    }
  }

  function run(hertz: number) {
    if (!hertz) return;
    if (!simulator) {
      createSimulator();
    }

    intervalId = setInterval(() => {
      try {
        editor.highlightLine(simulator!.current_line());
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

  function step() {
    if (!simulator) {
      createSimulator();
    }

    editor.highlightLine(simulator!.current_line());
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

  onMount(() => {
    resizeMemory(memSize);
  });

  function resizeMemory(size: number | null) {
    if (simulator) return;
    if (!size) {
      memoryError = "Invalid memory size";
      return;
    }

    memoryError = "";
    memory.resize(size);
  }
</script>

<div class="container">
  <div class="editor">
    <Editor bind:this={editor} />
  </div>
  <div class="sidebar">
    <div class="controls">
      <label>
        Memory size (bytes)
        <input
          type="number"
          bind:value={memSize}
          oninput={() => {
            resizeMemory(memSize);
          }}
        />
        <span>{memoryError}</span>
      </label>
      <br />
      <label>
        Run speed (Hz)
        <input type="number" bind:value={hertz} />
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
    </div>
    <Registers bind:this={registers} {diff} />
    <Memory bind:this={memory} {diff} />
  </div>
</div>

<style>
  .container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: row;
  }

  .editor {
    position: relative;
    width: 40%;
  }

  .sidebar {
    display: flex;
    flex-direction: column;
    flex: 1;
    height: 100vh;
    font-size: large;
    button {
      font-size: medium;
    }
  }
</style>
