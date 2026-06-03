#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::operands::{reg::Reg, registers::QwordReg, sizeparams::OpSize};

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemDiff {
    pub address: usize,
    pub value: u8,
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Rip,
    Flags,
}

impl<S: OpSize> From<Reg<S>> for DiffReg {
    fn from(value: Reg<S>) -> Self {
        let Reg::Qword(whole_reg) = value.to_whole_reg();

        match whole_reg {
            QwordReg::Rax => Self::Rax,
            QwordReg::Rbx => Self::Rbx,
            QwordReg::Rcx => Self::Rcx,
            QwordReg::Rdx => Self::Rdx,
            QwordReg::Rsi => Self::Rsi,
            QwordReg::Rdi => Self::Rdi,
            QwordReg::Rsp => Self::Rsp,
            QwordReg::Rbp => Self::Rbp,
            QwordReg::R8 => Self::R8,
            QwordReg::R9 => Self::R9,
            QwordReg::R10 => Self::R10,
            QwordReg::R11 => Self::R11,
            QwordReg::R12 => Self::R12,
            QwordReg::R13 => Self::R13,
            QwordReg::R14 => Self::R14,
            QwordReg::R15 => Self::R15,
        }
    }
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegDiff {
    pub reg: DiffReg,
    pub value: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Diff {
    Reg(RegDiff),
    Mem(Vec<MemDiff>),
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StateDiff {
    pub reg_diffs: Vec<RegDiff>,
    pub mem_diffs: Vec<MemDiff>,
}

#[cfg(feature = "wasm-bindgen")]
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
impl StateDiff {
    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
    // Fine to allow since wasm_bindgen does not expose derived trait methods.
    #[allow(clippy::should_implement_trait)]
    #[must_use]
    pub fn default() -> Self {
        <Self as Default>::default()
    }
}

impl StateDiff {
    pub fn push(&mut self, diff: Diff) {
        match diff {
            Diff::Reg(diff) => {
                self.reg_diffs.push(diff);
            }
            Diff::Mem(mut diffs) => {
                self.mem_diffs.append(&mut diffs);
            }
        }
    }
}
