use crate::operands::{Reg, RegOrImm32, RegOrImm64};

#[derive(Debug, PartialEq)]
pub enum Instr {
    Mov { dest: Reg, src: RegOrImm64 },
    Add { dest: Reg, src: RegOrImm32 },
    Sub { dest: Reg, src: RegOrImm32 },
    Xor { dest: Reg, src: RegOrImm32 },
    Jmp { dest: usize },
    Cmp { dest: Reg, src: RegOrImm32 },
    Je { dest: usize },
    Jne { dest: usize },
    Ja { dest: usize },
    Jae { dest: usize },
    Jb { dest: usize },
    Jbe { dest: usize },
    Jg { dest: usize },
    Jge { dest: usize },
    Jl { dest: usize },
    Jle { dest: usize },
}
