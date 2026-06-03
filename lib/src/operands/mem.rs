#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    operands::{
        immediates::{Bits, Imm32},
        registers::{BaseReg, IndexReg},
        sizeparams::OpSize,
        sizes::QDWB,
    },
    tokens::TokenType,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scale {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
}

impl TryFrom<u64> for Scale {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
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
pub struct Mem<S>
where
    S: OpSize,
{
    pub base: Option<BaseReg>,
    pub index: Option<Index>,
    pub disp: Option<Imm32>,
    pub size: Size,
    _size_marker: std::marker::PhantomData<S>,
}

impl<S> Mem<S>
where
    S: OpSize,
{
    pub fn new(
        base: Option<BaseReg>,
        index: Option<Index>,
        disp: Option<Imm32>,
        size: Size,
    ) -> Result<Self, ()> {
        // Check if the size is valid for S
        if S::QSizeT::try_from(size).is_ok()
            || S::DSizeT::try_from(size).is_ok()
            || S::WSizeT::try_from(size).is_ok()
            || S::BSizeT::try_from(size).is_ok()
        {
            Ok(Mem {
                base,
                index,
                disp,
                size,
                _size_marker: std::marker::PhantomData,
            })
        } else {
            Err(())
        }
    }

    pub fn try_from_other<S2: OpSize>(value: &Mem<S2>) -> Result<Self, ()> {
        Self::new(value.base, value.index, value.disp, value.size)
    }

    pub fn try_into_other<S2: OpSize>(self) -> Result<Mem<S2>, ()> {
        Mem::<S2>::try_from_other(&self)
    }

    pub fn into_all(self) -> Mem<QDWB> {
        Mem {
            base: self.base,
            index: self.index,
            disp: self.disp,
            size: self.size,
            _size_marker: std::marker::PhantomData,
        }
    }
}
