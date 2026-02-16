use crate::operands::Reg;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Mov,
    Add,
    Sub,
    Xor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Opcode(Opcode),
    Reg(Reg),
    Imm(u64),
}
