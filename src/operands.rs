use std::{fmt::Display, num::TryFromIntError};

use crate::tokens::Token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Imm32(pub u32);

impl From<Imm32> for u64 {
    fn from(value: Imm32) -> Self {
        // Sign extend the number.
        i64::from(value.0.cast_signed()).cast_unsigned()
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

impl From<Imm64> for u64 {
    fn from(value: Imm64) -> Self {
        value.0
    }
}

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

impl From<IndexReg> for Reg {
    fn from(value: IndexReg) -> Self {
        type IR = IndexReg;
        match value {
            IR::Rax => Self::Rax,
            IR::Rbx => Self::Rbx,
            IR::Rcx => Self::Rcx,
            IR::Rdx => Self::Rdx,
            IR::Rsi => Self::Rsi,
            IR::Rdi => Self::Rdi,
            IR::Rbp => Self::Rbp,
            IR::R8 => Self::R8,
            IR::R9 => Self::R9,
            IR::R10 => Self::R10,
            IR::R11 => Self::R11,
            IR::R12 => Self::R12,
            IR::R13 => Self::R13,
            IR::R14 => Self::R14,
            IR::R15 => Self::R15,
        }
    }
}

impl TryFrom<Operand> for Reg {
    type Error = ();
    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Reg(reg) => Ok(reg),
            _ => Err(()),
        }
    }
}

// Reg without rsp.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndexReg {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
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

impl TryFrom<Reg> for IndexReg {
    type Error = ();
    fn try_from(value: Reg) -> Result<Self, Self::Error> {
        type R = Reg;
        match value {
            R::Rax => Ok(Self::Rax),
            R::Rbx => Ok(Self::Rbx),
            R::Rcx => Ok(Self::Rcx),
            R::Rdx => Ok(Self::Rdx),
            R::Rsi => Ok(Self::Rsi),
            R::Rdi => Ok(Self::Rdi),
            R::Rbp => Ok(Self::Rbp),
            R::R8 => Ok(Self::R8),
            R::R9 => Ok(Self::R9),
            R::R10 => Ok(Self::R10),
            R::R11 => Ok(Self::R11),
            R::R12 => Ok(Self::R12),
            R::R13 => Ok(Self::R13),
            R::R14 => Ok(Self::R14),
            R::R15 => Ok(Self::R15),
            R::Rsp => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scale {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
}

impl TryFrom<i128> for Scale {
    type Error = ();
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            4 => Ok(Self::Four),
            8 => Ok(Self::Eight),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Byte,
    Word,
    Dword,
    Qword,
}

impl TryFrom<Token> for Size {
    type Error = ();
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Byte => Ok(Self::Byte),
            Token::Word => Ok(Self::Word),
            Token::Dword => Ok(Self::Dword),
            Token::Qword => Ok(Self::Qword),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Index {
    pub index: IndexReg,
    pub scale: Scale,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mem {
    pub base: Option<Reg>,
    pub index: Option<Index>,
    pub disp: Option<Imm32>,
    pub size: Size,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RMI32 {
    Reg(Reg),
    Mem(Mem),
    Imm(Imm32),
}

impl TryFrom<Operand> for RMI32 {
    type Error = TryFromIntError;
    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Reg(reg) => Ok(Self::Reg(reg)),
            Operand::Mem(mem) => Ok(Self::Mem(mem)),
            Operand::Imm(imm) => Ok(Self::Imm(Imm32::try_from(imm)?)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RMI64 {
    Reg(Reg),
    Mem(Mem),
    Imm(Imm64),
}

impl TryFrom<Operand> for RMI64 {
    type Error = TryFromIntError;
    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Reg(reg) => Ok(Self::Reg(reg)),
            Operand::Mem(mem) => Ok(Self::Mem(mem)),
            Operand::Imm(imm) => Ok(Self::Imm(Imm64::try_from(imm)?)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RM {
    Reg(Reg),
    Mem(Mem),
}

impl From<Reg> for RM {
    fn from(value: Reg) -> Self {
        RM::Reg(value)
    }
}

impl From<Mem> for RM {
    fn from(value: Mem) -> Self {
        RM::Mem(value)
    }
}

impl TryFrom<Operand> for RM {
    type Error = ();
    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Reg(reg) => Ok(Self::Reg(reg)),
            Operand::Mem(mem) => Ok(Self::Mem(mem)),
            Operand::Imm(_) => Err(()),
        }
    }
}

pub enum SimulatorOperand {
    Reg(Reg),
    Mem(Mem),
    Imm(u64),
}

impl From<Imm32> for SimulatorOperand {
    fn from(value: Imm32) -> Self {
        Self::Imm(u64::from(value))
    }
}

impl From<Imm64> for SimulatorOperand {
    fn from(value: Imm64) -> Self {
        Self::Imm(u64::from(value))
    }
}

impl From<RMI32> for SimulatorOperand {
    fn from(value: RMI32) -> Self {
        match value {
            RMI32::Reg(reg) => Self::Reg(reg),
            RMI32::Mem(mem) => Self::Mem(mem),
            RMI32::Imm(imm) => Self::Imm(u64::from(imm)),
        }
    }
}

impl From<RMI64> for SimulatorOperand {
    fn from(value: RMI64) -> Self {
        match value {
            RMI64::Reg(reg) => Self::Reg(reg),
            RMI64::Mem(mem) => Self::Mem(mem),
            RMI64::Imm(imm) => Self::Imm(u64::from(imm)),
        }
    }
}

impl From<RI32> for SimulatorOperand {
    fn from(value: RI32) -> Self {
        match value {
            RI32::Reg(reg) => Self::Reg(reg),
            RI32::Imm(imm) => Self::Imm(u64::from(imm)),
        }
    }
}

impl<T: Into<RM>> From<T> for SimulatorOperand {
    fn from(value: T) -> Self {
        match Into::into(value) {
            RM::Reg(reg) => SimulatorOperand::Reg(reg),
            RM::Mem(mem) => SimulatorOperand::Mem(mem),
        }
    }
}

pub enum Operand {
    Reg(Reg),
    Mem(Mem),
    Imm(i128),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RI32 {
    Reg(Reg),
    Imm(Imm32),
}

#[derive(Debug)]
pub enum RIConversionError {
    NotRegOrImm,
    ValueOutOfRange,
}

impl Display for RIConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotRegOrImm => write!(f, "expected register or immediate"),
            Self::ValueOutOfRange => write!(f, "value out of range for dword"),
        }
    }
}

impl From<TryFromIntError> for RIConversionError {
    fn from(_value: TryFromIntError) -> Self {
        Self::ValueOutOfRange
    }
}

impl TryFrom<Operand> for RI32 {
    type Error = RIConversionError;
    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Reg(reg) => Ok(Self::Reg(reg)),
            Operand::Imm(imm) => Ok(Self::Imm(Imm32::try_from(imm)?)),
            Operand::Mem(_) => Err(RIConversionError::NotRegOrImm),
        }
    }
}
