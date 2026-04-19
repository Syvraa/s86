use crate::operands::{Mem, RI32, RMI32, RMI64, Reg};

#[derive(Debug, PartialEq)]
pub struct Instr {
    pub line: usize,
    pub kind: InstrKind,
}

#[derive(Debug, PartialEq)]
pub enum InstrKind {
    Mov { dest: Reg, src: RMI64 },
    MovMem { dest: Mem, src: RI32 },
    Add { dest: Reg, src: RMI32 },
    AddMem { dest: Mem, src: RI32 },
    Sub { dest: Reg, src: RMI32 },
    SubMem { dest: Mem, src: RI32 },
    Xor { dest: Reg, src: RMI32 },
    XorMem { dest: Mem, src: RI32 },
    Cmp { dest: Reg, src: RMI32 },
    CmpMem { dest: Mem, src: RI32 },
    Jmp { dest: usize },
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
