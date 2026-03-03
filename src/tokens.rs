use crate::operands::{Label, Reg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Mov,
    Add,
    Sub,
    Xor,
    Jmp,
}

impl Opcode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mov => "mov",
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Xor => "xor",
            Self::Jmp => "jmp",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Opcode(Opcode),
    Reg(Reg),
    Imm(i128),
    Label(Label),
    Sublabel(Label),
    Colon,
    Comma,
}
