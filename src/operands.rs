use std::num::TryFromIntError;

use crate::tokens::Token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Imm32(pub u32);

impl From<Imm32> for u64 {
    fn from(value: Imm32) -> Self {
        u64::from(value.0)
    }
}

impl Imm32 {
    pub fn sign_extend(self) -> u64 {
        // Treat self.0 as a signed integer, then convert it to i64, otherwise it won't get
        // sign extended.
        i64::from(self.0.cast_signed()).cast_unsigned()
    }
}

impl TryFrom<i128> for Imm32 {
    type Error = TryFromIntError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value < 0 {
            Ok(Imm32(i32::try_from(value)?.cast_unsigned()))
        } else {
            Ok(Imm32(u32::try_from(value)?))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Imm64(pub u64);

impl TryFrom<i128> for Imm64 {
    type Error = TryFromIntError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value < 0 {
            Ok(Imm64(i64::try_from(value)?.cast_unsigned()))
        } else {
            Ok(Imm64(u64::try_from(value)?))
        }
    }
}

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
            Token::Imm(val) => Ok(Self::Imm(val.try_into()?)),
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
    type Error = TryFromIntError;
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match *token {
            Token::Reg(reg) => Ok(Self::Reg(reg)),
            Token::Imm(val) => Ok(Self::Imm(val.try_into()?)),
            _ => unreachable!("invalid token (the parser should handle this)"),
        }
    }
}
