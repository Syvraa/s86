use crate::operands::{
    mem::Mem,
    operand::Operand,
    reg::Reg,
    sizes::{D, Q, QDWB},
};

#[derive(Debug, PartialEq)]
pub struct Instr {
    pub line: usize,
    pub kind: InstrKind,
}

#[derive(Debug, PartialEq)]
pub enum InstrKind {
    Mov {
        dest: Reg<QDWB>,
        src: Operand<QDWB, QDWB, Q>,
    },
    MovMem {
        dest: Mem<QDWB>,
        src: Operand<QDWB, !, D>,
    },
    Add {
        dest: Reg<QDWB>,
        src: Operand<QDWB, QDWB, D>,
    },
    AddMem {
        dest: Mem<QDWB>,
        src: Operand<QDWB, !, D>,
    },
    Sub {
        dest: Reg<QDWB>,
        src: Operand<QDWB, QDWB, D>,
    },
    SubMem {
        dest: Mem<QDWB>,
        src: Operand<QDWB, !, D>,
    },
    Xor {
        dest: Reg<QDWB>,
        src: Operand<QDWB, QDWB, D>,
    },
    XorMem {
        dest: Mem<QDWB>,
        src: Operand<QDWB, !, D>,
    },
    Cmp {
        dest: Reg<QDWB>,
        src: Operand<QDWB, QDWB, D>,
    },
    CmpMem {
        dest: Mem<QDWB>,
        src: Operand<QDWB, !, D>,
    },
    Jmp {
        dest: usize,
    },
    Je {
        dest: usize,
    },
    Jne {
        dest: usize,
    },
    Ja {
        dest: usize,
    },
    Jae {
        dest: usize,
    },
    Jb {
        dest: usize,
    },
    Jbe {
        dest: usize,
    },
    Jg {
        dest: usize,
    },
    Jge {
        dest: usize,
    },
    Jl {
        dest: usize,
    },
    Jle {
        dest: usize,
    },
}
