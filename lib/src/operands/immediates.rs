use std::num::TryFromIntError;

use crate::operands::operand::OperandConversionError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Imm32(pub u32);

impl From<Imm32> for u64 {
    /// Sign extends the number.
    fn from(value: Imm32) -> Self {
        i64::from(value.0.cast_signed()).cast_unsigned()
    }
}

impl From<Imm32> for Imm64 {
    fn from(value: Imm32) -> Self {
        Imm64(u64::from(value))
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

impl TryFrom<Imm64> for Imm32 {
    type Error = OperandConversionError;

    fn try_from(value: Imm64) -> Result<Self, Self::Error> {
        if value.0.cast_signed() < 0 {
            Ok(Imm32(
                i32::try_from(value.0.cast_signed())
                    .map_err(|_| OperandConversionError::ImmediateOutOfRangeForDword)?
                    .cast_unsigned(),
            ))
        } else {
            Ok(Imm32(u32::try_from(value.0).map_err(|_| {
                OperandConversionError::ImmediateOutOfRangeForDword
            })?))
        }
    }
}

impl TryFrom<Imm64> for ! {
    type Error = OperandConversionError;

    fn try_from(_: Imm64) -> Result<Self, Self::Error> {
        Err(OperandConversionError::WrongOperand)
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

pub trait Bits {
    /// How many bits this number takes.
    fn bits(self) -> u32;
}

impl Bits for Imm64 {
    fn bits(self) -> u32 {
        if self.0 == 0 { 8 } else { self.0.ilog2() + 1 }
    }
}

impl Bits for Imm32 {
    fn bits(self) -> u32 {
        if self.0 == 0 { 8 } else { self.0.ilog2() + 1 }
    }
}

impl Bits for ! {
    fn bits(self) -> u32 {
        unreachable!()
    }
}
