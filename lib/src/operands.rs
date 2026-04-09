use std::{
    convert::Infallible,
    num::TryFromIntError,
    ops::{FromResidual, Try},
};

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::tokens::TokenType;

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
pub enum QwordReg {
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
pub enum DwordReg {
    Eax,
    Ebx,
    Ecx,
    Edx,
    Esi,
    Edi,
    Esp,
    Ebp,
    R8d,
    R9d,
    R10d,
    R11d,
    R12d,
    R13d,
    R14d,
    R15d,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WordReg {
    Ax,
    Bx,
    Cx,
    Dx,
    Si,
    Di,
    Sp,
    Bp,
    R8w,
    R9w,
    R10w,
    R11w,
    R12w,
    R13w,
    R14w,
    R15w,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ByteReg {
    Ah,
    Al,
    Bh,
    Bl,
    Ch,
    Cl,
    Dh,
    Dl,
    Sil,
    Dil,
    Spl,
    Bpl,
    R8b,
    R9b,
    R10b,
    R11b,
    R12b,
    R13b,
    R14b,
    R15b,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reg {
    Qword(QwordReg),
    Dword(DwordReg),
    Word(WordReg),
    Byte(ByteReg),
}

impl Reg {
    pub fn size(self) -> Size {
        match self {
            Self::Qword(_) => Size::Qword,
            Self::Dword(_) => Size::Dword,
            Self::Word(_) => Size::Word,
            Self::Byte(_) => Size::Byte,
        }
    }
}

impl From<IndexReg> for Reg {
    fn from(value: IndexReg) -> Self {
        match value {
            IndexReg::Qword(reg) => Reg::Qword(QwordReg::from(reg)),
            IndexReg::Dword(reg) => Reg::Dword(DwordReg::from(reg)),
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QwordIndexReg {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DwordIndexReg {
    Eax,
    Ebx,
    Ecx,
    Edx,
    Esi,
    Edi,
    Ebp,
    R8d,
    R9d,
    R10d,
    R11d,
    R12d,
    R13d,
    R14d,
    R15d,
}

impl From<QwordIndexReg> for QwordReg {
    fn from(value: QwordIndexReg) -> Self {
        match value {
            QwordIndexReg::Rax => Self::Rax,
            QwordIndexReg::Rbx => Self::Rbx,
            QwordIndexReg::Rcx => Self::Rcx,
            QwordIndexReg::Rdx => Self::Rdx,
            QwordIndexReg::Rsi => Self::Rsi,
            QwordIndexReg::Rdi => Self::Rdi,
            QwordIndexReg::Rbp => Self::Rbp,
            QwordIndexReg::R8 => Self::R8,
            QwordIndexReg::R9 => Self::R9,
            QwordIndexReg::R10 => Self::R10,
            QwordIndexReg::R11 => Self::R11,
            QwordIndexReg::R12 => Self::R12,
            QwordIndexReg::R13 => Self::R13,
            QwordIndexReg::R14 => Self::R14,
            QwordIndexReg::R15 => Self::R15,
        }
    }
}

impl From<DwordIndexReg> for DwordReg {
    fn from(value: DwordIndexReg) -> Self {
        match value {
            DwordIndexReg::Eax => Self::Eax,
            DwordIndexReg::Ebx => Self::Ebx,
            DwordIndexReg::Ecx => Self::Ecx,
            DwordIndexReg::Edx => Self::Edx,
            DwordIndexReg::Esi => Self::Esi,
            DwordIndexReg::Edi => Self::Edi,
            DwordIndexReg::Ebp => Self::Ebp,
            DwordIndexReg::R8d => Self::R8d,
            DwordIndexReg::R9d => Self::R9d,
            DwordIndexReg::R10d => Self::R10d,
            DwordIndexReg::R11d => Self::R11d,
            DwordIndexReg::R12d => Self::R12d,
            DwordIndexReg::R13d => Self::R13d,
            DwordIndexReg::R14d => Self::R14d,
            DwordIndexReg::R15d => Self::R15d,
        }
    }
}

impl TryFrom<QwordReg> for QwordIndexReg {
    type Error = ();

    fn try_from(value: QwordReg) -> Result<Self, Self::Error> {
        match value {
            QwordReg::Rax => Ok(Self::Rax),
            QwordReg::Rbx => Ok(Self::Rbx),
            QwordReg::Rcx => Ok(Self::Rcx),
            QwordReg::Rdx => Ok(Self::Rdx),
            QwordReg::Rsi => Ok(Self::Rsi),
            QwordReg::Rdi => Ok(Self::Rdi),
            QwordReg::Rbp => Ok(Self::Rbp),
            QwordReg::R8 => Ok(Self::R8),
            QwordReg::R9 => Ok(Self::R9),
            QwordReg::R10 => Ok(Self::R10),
            QwordReg::R11 => Ok(Self::R11),
            QwordReg::R12 => Ok(Self::R12),
            QwordReg::R13 => Ok(Self::R13),
            QwordReg::R14 => Ok(Self::R14),
            QwordReg::R15 => Ok(Self::R15),
            QwordReg::Rsp => Err(()),
        }
    }
}

impl TryFrom<DwordReg> for DwordIndexReg {
    type Error = ();

    fn try_from(value: DwordReg) -> Result<Self, Self::Error> {
        match value {
            DwordReg::Eax => Ok(Self::Eax),
            DwordReg::Ebx => Ok(Self::Ebx),
            DwordReg::Ecx => Ok(Self::Ecx),
            DwordReg::Edx => Ok(Self::Edx),
            DwordReg::Esi => Ok(Self::Esi),
            DwordReg::Edi => Ok(Self::Edi),
            DwordReg::Ebp => Ok(Self::Ebp),
            DwordReg::R8d => Ok(Self::R8d),
            DwordReg::R9d => Ok(Self::R9d),
            DwordReg::R10d => Ok(Self::R10d),
            DwordReg::R11d => Ok(Self::R11d),
            DwordReg::R12d => Ok(Self::R12d),
            DwordReg::R13d => Ok(Self::R13d),
            DwordReg::R14d => Ok(Self::R14d),
            DwordReg::R15d => Ok(Self::R15d),
            DwordReg::Esp => Err(()),
        }
    }
}

// Reg without rsp.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndexReg {
    Qword(QwordIndexReg),
    Dword(DwordIndexReg),
}

impl TryFrom<Reg> for IndexReg {
    type Error = ();

    fn try_from(value: Reg) -> Result<Self, Self::Error> {
        match value {
            Reg::Qword(reg) => Ok(Self::Qword(QwordIndexReg::try_from(reg)?)),
            Reg::Dword(reg) => Ok(Self::Dword(DwordIndexReg::try_from(reg)?)),
            _ => Err(()),
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
pub enum Size {
    Byte,
    Word,
    Dword,
    Qword,
}

impl Bits for Size {
    /// How many bits the Size represents.
    fn bits(self) -> u32 {
        match self {
            Size::Byte => 8,
            Size::Word => 16,
            Size::Dword => 32,
            Size::Qword => 64,
        }
    }
}

impl TryFrom<TokenType> for Size {
    type Error = ();

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::Byte => Ok(Self::Byte),
            TokenType::Word => Ok(Self::Word),
            TokenType::Dword => Ok(Self::Dword),
            TokenType::Qword => Ok(Self::Qword),
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
pub enum BaseReg {
    Qword(QwordReg),
    Dword(DwordReg),
}

impl From<BaseReg> for Reg {
    fn from(value: BaseReg) -> Self {
        match value {
            BaseReg::Qword(reg) => Self::Qword(reg),
            BaseReg::Dword(reg) => Self::Dword(reg),
        }
    }
}

impl TryFrom<Reg> for BaseReg {
    type Error = ();

    fn try_from(value: Reg) -> Result<Self, Self::Error> {
        match value {
            Reg::Qword(reg) => Ok(Self::Qword(reg)),
            Reg::Dword(reg) => Ok(Self::Dword(reg)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mem {
    pub base: Option<BaseReg>,
    pub index: Option<Index>,
    pub disp: Option<Imm32>,
    pub size: Size,
}

pub enum OperandWithImmError {
    WrongOperand,
    ImmediateOutOfRange,
}

impl From<TryFromIntError> for OperandWithImmError {
    fn from(_: TryFromIntError) -> Self {
        Self::ImmediateOutOfRange
    }
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

impl TryFrom<Option<Operand>> for RMI32 {
    type Error = OperandWithImmError;

    fn try_from(value: Option<Operand>) -> Result<Self, Self::Error> {
        match value {
            Some(operand) => Ok(Self::try_from(operand)?),
            None => Err(OperandWithImmError::ImmediateOutOfRange),
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

impl TryFrom<Option<Operand>> for RMI64 {
    type Error = OperandWithImmError;

    fn try_from(value: Option<Operand>) -> Result<Self, Self::Error> {
        match value {
            Some(operand) => Ok(Self::try_from(operand)?),
            None => Err(OperandWithImmError::ImmediateOutOfRange),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RM {
    Reg(Reg),
    Mem(Mem),
}

impl RM {
    pub fn size(&self) -> Size {
        match self {
            Self::Reg(reg) => reg.size(),
            Self::Mem(mem) => mem.size,
        }
    }
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

impl TryFrom<Option<Operand>> for RM {
    type Error = ();

    fn try_from(value: Option<Operand>) -> Result<Self, Self::Error> {
        match value {
            Some(Operand::Reg(reg)) => Ok(Self::Reg(reg)),
            Some(Operand::Mem(mem)) => Ok(Self::Mem(mem)),
            _ => Err(()),
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

impl<T> From<T> for SimulatorOperand
where
    T: Into<RM>,
{
    fn from(value: T) -> Self {
        match value.into() {
            RM::Reg(reg) => Self::Reg(reg),
            RM::Mem(mem) => Self::Mem(mem),
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Reg(Reg),
    Mem(Mem),
    Imm(i128),
}

/// Contains `Ok(Some(Operand))` if parsing was successful.
/// Contains `Ok(None)` if the parsed token was not a valid `Operand` (or there was no token
/// remaining).
/// Contains `ParsingError` if an error occured during parsing (for example, a memory operand or a
/// negative number could not be parsed). In that case, the error was already pushed to `self.errors`,
/// so you should just use `?` to return None.
// I know this is a really weird type but this was the most convenient way I could come up for
// having 2 error states while making the api convenient (just use ? if there is an error, nothing
// more convenient).
pub enum OperandParseResult {
    Ok(Option<Operand>),
    ParsingError,
}

impl FromResidual for OperandParseResult {
    fn from_residual(_: <Self as Try>::Residual) -> Self {
        Self::ParsingError
    }
}

impl Try for OperandParseResult {
    type Output = Option<Operand>;
    // Basically, this can only be None.
    // TODO: Change this to ! once it is stabilized so it looks prettier.
    type Residual = Option<Infallible>;

    fn from_output(output: Self::Output) -> Self {
        Self::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Ok(value) => std::ops::ControlFlow::Continue(value),
            Self::ParsingError => std::ops::ControlFlow::Break(None),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RI32 {
    Reg(Reg),
    Imm(Imm32),
}

impl TryFrom<Operand> for RI32 {
    type Error = OperandWithImmError;

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Reg(reg) => Ok(Self::Reg(reg)),
            Operand::Imm(imm) => Ok(Self::Imm(Imm32::try_from(imm)?)),
            Operand::Mem(_) => Err(OperandWithImmError::WrongOperand),
        }
    }
}

impl TryFrom<Option<Operand>> for RI32 {
    type Error = OperandWithImmError;

    fn try_from(value: Option<Operand>) -> Result<Self, Self::Error> {
        match value {
            Some(Operand::Reg(reg)) => Ok(Self::Reg(reg)),
            Some(Operand::Imm(imm)) => Ok(Self::Imm(Imm32::try_from(imm)?)),
            _ => Err(OperandWithImmError::WrongOperand),
        }
    }
}

pub trait Bits {
    fn bits(self) -> u32;
}

impl Bits for i128 {
    /// How many bits this number takes.
    #[allow(clippy::comparison_chain)]
    fn bits(self) -> u32 {
        if self == 0 {
            8
        } else if self < 0 {
            if i8::try_from(self).is_ok() {
                8
            } else if i16::try_from(self).is_ok() {
                16
            } else if i32::try_from(self).is_ok() {
                32
            } else if i64::try_from(self).is_ok() {
                64
            } else {
                128
            }
        } else {
            self.cast_unsigned().ilog2() + 1
        }
    }
}
