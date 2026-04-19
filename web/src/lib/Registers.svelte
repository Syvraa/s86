<script lang="ts">
  import { DiffReg, type StateDiff } from "s86-lib";

  let { diff }: { diff: StateDiff } = $props();
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

  $effect(() => {
    for (const reg_diff of diff.reg_diffs) {
      registers[reg_diff.reg] = reg_diff.value;
    }
  });

  export function reset() {
    Object.assign(registers, {
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
  }
</script>

Rax {registers[DiffReg.Rax]}
<br />
Rbx {registers[DiffReg.Rbx]}
<br />
Rcx {registers[DiffReg.Rcx]}
<br />
Rdx {registers[DiffReg.Rdx]}
<br />
Rsi {registers[DiffReg.Rsi]}
<br />
Rdi {registers[DiffReg.Rdi]}
<br />
Rsp {registers[DiffReg.Rsp]}
<br />
Rbp {registers[DiffReg.Rbp]}
<br />
R8 {registers[DiffReg.R8]}
<br />
R9 {registers[DiffReg.R9]}
<br />
R10 {registers[DiffReg.R10]}
<br />
R11 {registers[DiffReg.R11]}
<br />
R12 {registers[DiffReg.R12]}
<br />
R13 {registers[DiffReg.R13]}
<br />
R14 {registers[DiffReg.R14]}
<br />
R15 {registers[DiffReg.R15]}
