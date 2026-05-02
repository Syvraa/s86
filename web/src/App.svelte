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

  let simulator: Simulator | null = $state(null);
  let registers: Registers;
  let memory: Memory;
  let editor: Editor;
  let hertz = $state(2);
  let memSize = $state(64);
  let intervalId = $state(0);
  let memoryError = $state("");
  let timeoutId: number | undefined = $state(undefined);

  function createSimulator() {
    if (!memSize) return;
    try {
      simulator = new Simulator(editor.getContent(), memSize);
      editor.clearErrors();
    } catch (errs) {
      editor.showErrors(errs as SyntaxError[]);
      throw {};
    }
  }

  function checkForErrors() {
    if (simulator) return;
    try {
      new Simulator(editor.getContent(), 0);
      editor.clearErrors();
    } catch (errs) {
      editor.showErrors(errs as SyntaxError[]);
    }
  }

  function stepSimulator(onEnd?: () => void) {
    if (!simulator) return;

    if (simulator.is_at_end() && onEnd) {
      onEnd();
    }

    try {
      editor.highlightLine(simulator!.current_line());
      const diff = simulator.step();
      registers.update(diff);
      memory.update(diff);
    } catch (err) {
      const error = err as SimulatorError;
      editor.highlightSimulatorError(simulator!.current_line()!, error);
    }
  }

  function run(hertz: number) {
    if (!hertz) return;
    if (!simulator) {
      try {
        createSimulator();
      } catch {
        return;
      }
    }

    intervalId = setInterval(() => {
      stepSimulator(() => {
        clearInterval(intervalId);
      });
    }, 1000 / hertz);
  }

  function step() {
    if (!simulator) {
      try {
        createSimulator();
      } catch {
        return;
      }
    }

    stepSimulator();
  }

  function setRsp() {
    registers.update({
      reg_diffs: [{ reg: DiffReg.Rsp, value: BigInt(memSize - 1) } as RegDiff],
      mem_diffs: [] as MemDiff[],
    } as StateDiff);
    registers.update({
      reg_diffs: [] as RegDiff[],
      mem_diffs: [] as MemDiff[],
    } as StateDiff);
  }

  function reset() {
    simulator = null;
    if (intervalId) clearInterval(intervalId);
    registers.reset();
    setRsp();
    memory.reset();
    editor.reset();
  }

  onMount(() => {
    resizeMemory(memSize);
    setRsp();
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
    <Editor
      bind:this={editor}
      onchange={() => {
        clearTimeout(timeoutId);
        timeoutId = setTimeout(checkForErrors, 2000);
      }}
    />
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
            if (memSize) {
              setRsp();
            }
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
    <Registers bind:this={registers} />
    <Memory bind:this={memory} />
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
