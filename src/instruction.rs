use crate::operands::{Reg, RegOrImm32, RegOrImm64};

#[derive(Debug, PartialEq)]
pub enum Instr {
    Mov { dest: Reg, src: RegOrImm64 },
    Add { dest: Reg, src: RegOrImm32 },
    Sub { dest: Reg, src: RegOrImm32 },
    Xor { dest: Reg, src: RegOrImm32 },
}
