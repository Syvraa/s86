#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::operands::{ByteReg, DwordReg, QwordReg, Reg, Size, WordReg};

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Clone, Copy)]
pub struct MemDiff {
    pub address: usize,
    pub size: Size,
    pub value: u64,
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Clone, Copy)]
pub enum DiffReg {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rsp,
    Rbp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    Flags,
}

impl From<Reg> for DiffReg {
    fn from(value: Reg) -> Self {
        type R = Reg;
        type Q = QwordReg;
        type D = DwordReg;
        type W = WordReg;
        type B = ByteReg;
        match value {
            R::Qword(Q::Rax) | R::Dword(D::Eax) | R::Word(W::Ax) | R::Byte(B::Ah | B::Al) => {
                Self::Rax
            }
            R::Qword(Q::Rbx) | R::Dword(D::Ebx) | R::Word(W::Bx) | R::Byte(B::Bh | B::Bl) => {
                Self::Rbx
            }
            R::Qword(Q::Rcx) | R::Dword(D::Ecx) | R::Word(W::Cx) | R::Byte(B::Ch | B::Cl) => {
                Self::Rcx
            }
            R::Qword(Q::Rdx) | R::Dword(D::Edx) | R::Word(W::Dx) | R::Byte(B::Dh | B::Dl) => {
                Self::Rdx
            }
            R::Qword(Q::Rsi) | R::Dword(D::Esi) | R::Word(W::Si) | R::Byte(B::Sil) => Self::Rsi,
            R::Qword(Q::Rdi) | R::Dword(D::Edi) | R::Word(W::Di) | R::Byte(B::Dil) => Self::Rdi,
            R::Qword(Q::Rsp) | R::Dword(D::Esp) | R::Word(W::Sp) | R::Byte(B::Spl) => Self::Rsp,
            R::Qword(Q::Rbp) | R::Dword(D::Ebp) | R::Word(W::Bp) | R::Byte(B::Bpl) => Self::Rbp,
            R::Qword(Q::R8) | R::Dword(D::R8d) | R::Word(W::R8w) | R::Byte(B::R8b) => Self::R8,
            R::Qword(Q::R9) | R::Dword(D::R9d) | R::Word(W::R9w) | R::Byte(B::R9b) => Self::R9,
            R::Qword(Q::R10) | R::Dword(D::R10d) | R::Word(W::R10w) | R::Byte(B::R10b) => Self::R10,
            R::Qword(Q::R11) | R::Dword(D::R11d) | R::Word(W::R11w) | R::Byte(B::R11b) => Self::R11,
            R::Qword(Q::R12) | R::Dword(D::R12d) | R::Word(W::R12w) | R::Byte(B::R12b) => Self::R12,
            R::Qword(Q::R13) | R::Dword(D::R13d) | R::Word(W::R13w) | R::Byte(B::R13b) => Self::R13,
            R::Qword(Q::R14) | R::Dword(D::R14d) | R::Word(W::R14w) | R::Byte(B::R14b) => Self::R14,
            R::Qword(Q::R15) | R::Dword(D::R15d) | R::Word(W::R15w) | R::Byte(B::R15b) => Self::R15,
        }
    }
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Clone, Copy)]
pub struct RegDiff {
    pub reg: DiffReg,
    pub value: u64,
}

#[derive(Clone, Copy)]
pub enum Diff {
    Reg(RegDiff),
    Mem(MemDiff),
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(getter_with_clone))]
#[derive(Default)]
pub struct StateDiff {
    pub reg_diffs: Vec<RegDiff>,
    pub mem_diffs: Vec<MemDiff>,
}

impl StateDiff {
    pub fn push(&mut self, diff: Diff) {
        match diff {
            Diff::Reg(diff) => {
                self.reg_diffs.push(diff);
            }
            Diff::Mem(diff) => {
                self.mem_diffs.push(diff);
            }
        }
    }
}
