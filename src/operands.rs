use std::num::TryFromIntError;

use crate::tokens::Token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Imm32(pub u32);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Imm64(pub u64);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reg {
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegOrImm32 {
    Reg(Reg),
    Imm(Imm32),
}

impl TryFrom<&Token> for RegOrImm32 {
    type Error = TryFromIntError;
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match *token {
            Token::Reg(reg) => Ok(Self::Reg(reg)),
            Token::Imm(val) => Ok(Self::Imm(Imm32(u32::try_from(val)?))),
            _ => unreachable!("invalid token (the parser should handle this)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegOrImm64 {
    Reg(Reg),
    Imm(Imm64),
}

impl TryFrom<&Token> for RegOrImm64 {
    type Error = ();
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match *token {
            Token::Reg(reg) => Ok(Self::Reg(reg)),
            Token::Imm(val) => Ok(Self::Imm(Imm64(val))),
            _ => unreachable!("invalid token (the parser should handle this)"),
        }
    }
}
