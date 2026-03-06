use crate::operands::{Label, Reg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Mov,
    Add,
    Sub,
    Xor,
    Cmp,
    Jmp,
    Je,
    Jz,
    Jne,
    Jnz,
    Ja,
    Jnbe,
    Jae,
    Jnb,
    Jb,
    Jnae,
    Jbe,
    Jna,
    Jg,
    Jnle,
    Jge,
    Jnl,
    Jl,
    Jnge,
    Jle,
    Jng,
}

impl Opcode {
    pub fn as_str(self) -> &'static str {
        match self {
            Opcode::Mov => "mov",
            Opcode::Add => "add",
            Opcode::Sub => "sub",
            Opcode::Xor => "xor",
            Opcode::Cmp => "cmp",
            Opcode::Jmp => "jmp",
            Opcode::Je => "je",
            Opcode::Jz => "jz",
            Opcode::Jne => "jne",
            Opcode::Jnz => "jnz",
            Opcode::Ja => "ja",
            Opcode::Jnbe => "jnbe",
            Opcode::Jae => "jae",
            Opcode::Jnb => "jnb",
            Opcode::Jb => "jb",
            Opcode::Jnae => "jnae",
            Opcode::Jbe => "jbe",
            Opcode::Jna => "jna",
            Opcode::Jg => "jg",
            Opcode::Jnle => "jnle",
            Opcode::Jge => "jge",
            Opcode::Jnl => "jnl",
            Opcode::Jl => "jl",
            Opcode::Jnge => "jnge",
            Opcode::Jle => "jle",
            Opcode::Jng => "jng",
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
