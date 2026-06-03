use crate::operands::{
    immediates::{Bits, Imm64},
    mem::Size,
    registers::{ByteReg, DwordIndexReg, DwordReg, QwordIndexReg, QwordReg, WordReg},
};

pub trait OpSize {
    type QRegT: From<QwordReg>
        + Into<QwordReg>
        + TryInto<QwordReg>
        + From<Self::QRegT>
        + TryInto<QwordIndexReg>
        + std::fmt::Debug
        + Clone
        + Copy
        + PartialEq;
    type DRegT: From<DwordReg>
        + Into<DwordReg>
        + TryInto<DwordReg>
        + TryInto<DwordIndexReg>
        + std::fmt::Debug
        + Clone
        + Copy
        + PartialEq;
    type WRegT: From<WordReg>
        + Into<WordReg>
        + TryInto<WordReg>
        + std::fmt::Debug
        + Clone
        + Copy
        + PartialEq;
    type BRegT: From<ByteReg>
        + Into<ByteReg>
        + TryInto<ByteReg>
        + std::fmt::Debug
        + Clone
        + Copy
        + PartialEq;
    type QSizeT: TryFrom<Size>;
    type DSizeT: TryFrom<Size>;
    type WSizeT: TryFrom<Size>;
    type BSizeT: TryFrom<Size>;
}

pub trait ImmSize {
    type ImmT: TryFrom<Imm64> + Into<Imm64> + Clone + Copy + Bits;
}
