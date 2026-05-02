<script lang="ts">
  import { DiffReg, StateDiff } from "s86-lib";
  import Register from "./Register.svelte";
  import Flags from "./Flags.svelte";

  let registers: Record<DiffReg, bigint> = $state({
    [DiffReg.Rax]: 0n,
    [DiffReg.Rbx]: 0n,
    [DiffReg.Rcx]: 0n,
    [DiffReg.Rdx]: 0n,
    [DiffReg.Rsi]: 0n,
    [DiffReg.Rdi]: 0n,
    [DiffReg.Rsp]: 0n,
    [DiffReg.Rbp]: 0n,
    [DiffReg.R8]: 0n,
    [DiffReg.R9]: 0n,
    [DiffReg.R10]: 0n,
    [DiffReg.R11]: 0n,
    [DiffReg.R12]: 0n,
    [DiffReg.R13]: 0n,
    [DiffReg.R14]: 0n,
    [DiffReg.R15]: 0n,
    [DiffReg.Flags]: 0n,
  });
  let diff: StateDiff = $state(StateDiff.default());

  export function update(new_diff: StateDiff) {
    diff = new_diff;
    for (const reg_diff of new_diff.reg_diffs) {
      registers[reg_diff.reg] = reg_diff.value;
    }
  }

  export function reset() {
    registers = {
      [DiffReg.Rax]: 0n,
      [DiffReg.Rbx]: 0n,
      [DiffReg.Rcx]: 0n,
      [DiffReg.Rdx]: 0n,
      [DiffReg.Rsi]: 0n,
      [DiffReg.Rdi]: 0n,
      [DiffReg.Rsp]: 0n,
      [DiffReg.Rbp]: 0n,
      [DiffReg.R8]: 0n,
      [DiffReg.R9]: 0n,
      [DiffReg.R10]: 0n,
      [DiffReg.R11]: 0n,
      [DiffReg.R12]: 0n,
      [DiffReg.R13]: 0n,
      [DiffReg.R14]: 0n,
      [DiffReg.R15]: 0n,
      [DiffReg.Flags]: 0n,
    };
    diff = StateDiff.default();
  }
</script>

<div class="container">
  <div class="register">
    <Register
      name="rax"
      value={registers[DiffReg.Rax]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.Rax)}
    />
  </div>
  <div class="register">
    <Register
      name="rbx"
      value={registers[DiffReg.Rbx]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.Rbx)}
    />
  </div>
  <div class="register">
    <Register
      name="rcx"
      value={registers[DiffReg.Rcx]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.Rcx)}
    />
  </div>
  <div class="register">
    <Register
      name="rdx"
      value={registers[DiffReg.Rdx]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.Rdx)}
    />
  </div>
  <div class="register">
    <Register
      name="rsi"
      value={registers[DiffReg.Rsi]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.Rsi)}
    />
  </div>
  <div class="register">
    <Register
      name="rdi"
      value={registers[DiffReg.Rdi]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.Rdi)}
    />
  </div>
  <div class="register">
    <Register
      name="rsp"
      value={registers[DiffReg.Rsp]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.Rsp)}
    />
  </div>
  <div class="register">
    <Register
      name="rbp"
      value={registers[DiffReg.Rbp]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.Rbp)}
    />
  </div>
  <div class="register">
    <Register
      name="r8"
      value={registers[DiffReg.R8]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.R8)}
    />
  </div>
  <div class="register">
    <Register
      name="r9"
      value={registers[DiffReg.R9]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.R9)}
    />
  </div>
  <div class="register">
    <Register
      name="r10"
      value={registers[DiffReg.R10]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.R10)}
    />
  </div>
  <div class="register">
    <Register
      name="r11"
      value={registers[DiffReg.R11]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.R11)}
    />
  </div>
  <div class="register">
    <Register
      name="r12"
      value={registers[DiffReg.R12]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.R12)}
    />
  </div>
  <div class="register">
    <Register
      name="r13"
      value={registers[DiffReg.R13]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.R13)}
    />
  </div>
  <div class="register">
    <Register
      name="r14"
      value={registers[DiffReg.R14]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.R14)}
    />
  </div>
  <div class="register">
    <Register
      name="r15"
      value={registers[DiffReg.R15]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.R15)}
    />
  </div>
  <div class="register">
    <Flags
      value={registers[DiffReg.Flags]}
      highlight={diff.reg_diffs.some((d) => d.reg == DiffReg.Flags)}
    />
  </div>
</div>

<style>
  .container {
    display: flex;
    width: 100%;
    flex-direction: row;
    flex-wrap: wrap;
  }

  .register {
    font-family: monospace;
    min-width: 20ch;
  }
</style>
