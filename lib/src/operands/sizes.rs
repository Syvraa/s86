#![allow(clippy::upper_case_acronyms)]

use crate::operands::{
    immediates::{Imm32, Imm64},
    registers::{ByteReg, DwordReg, QwordReg, WordReg},
    sizeparams::{ImmSize, OpSize},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QDWB;
impl OpSize for QDWB {
    type QRegT = QwordReg;
    type DRegT = DwordReg;
    type WRegT = WordReg;
    type BRegT = ByteReg;
    type QSizeT = Q;
    type DSizeT = D;
    type WSizeT = W;
    type BSizeT = B;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QD;
impl OpSize for QD {
    type QRegT = QwordReg;
    type DRegT = DwordReg;
    type WRegT = !;
    type BRegT = !;
    type QSizeT = Q;
    type DSizeT = D;
    type WSizeT = !;
    type BSizeT = !;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q;
impl OpSize for Q {
    type QRegT = QwordReg;
    type DRegT = !;
    type WRegT = !;
    type BRegT = !;
    type QSizeT = Q;
    type DSizeT = !;
    type WSizeT = !;
    type BSizeT = !;
}
impl ImmSize for Q {
    type ImmT = Imm64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct D;
impl OpSize for D {
    type QRegT = !;
    type DRegT = DwordReg;
    type WRegT = !;
    type BRegT = !;
    type QSizeT = !;
    type DSizeT = D;
    type WSizeT = !;
    type BSizeT = !;
}
impl ImmSize for D {
    type ImmT = Imm32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct W;
impl OpSize for W {
    type QRegT = !;
    type DRegT = !;
    type WRegT = WordReg;
    type BRegT = !;
    type QSizeT = !;
    type DSizeT = !;
    type WSizeT = W;
    type BSizeT = !;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct B;
impl OpSize for B {
    type QRegT = !;
    type DRegT = !;
    type WRegT = !;
    type BRegT = ByteReg;
    type QSizeT = !;
    type DSizeT = !;
    type WSizeT = !;
    type BSizeT = B;
}

impl OpSize for ! {
    type QRegT = !;
    type DRegT = !;
    type WRegT = !;
    type BRegT = !;
    type QSizeT = !;
    type DSizeT = !;
    type WSizeT = !;
    type BSizeT = !;
}
impl ImmSize for ! {
    type ImmT = !;
}

impl TryFrom<super::mem::Size> for ! {
    type Error = ();

    fn try_from(_: super::mem::Size) -> Result<Self, Self::Error> {
        Err(())
    }
}
