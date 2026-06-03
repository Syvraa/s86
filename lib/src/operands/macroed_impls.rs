use crate::operands::{
    immediates::Imm64,
    mem::Size,
    registers::{ByteReg, DwordIndexReg, DwordReg, QwordIndexReg, QwordReg, WordReg},
    sizes::{B, D, Q, W},
};

macro_rules! from_for_never {
    ($ty:ty) => {
        impl From<$ty> for ! {
            fn from(_: $ty) -> Self {
                unreachable!()
            }
        }
    };
}

from_for_never!(QwordReg);
from_for_never!(DwordReg);
from_for_never!(WordReg);
from_for_never!(ByteReg);

macro_rules! from_never {
    ($ty:ty) => {
        impl From<!> for $ty {
            fn from(_: !) -> Self {
                unreachable!()
            }
        }
    };
}

from_never!(QwordReg);
from_never!(DwordReg);
from_never!(WordReg);
from_never!(ByteReg);
from_never!(QwordIndexReg);
from_never!(DwordIndexReg);
from_never!(Imm64);

macro_rules! try_from_size {
    ($ty:ty, $pat:pat) => {
        impl TryFrom<Size> for $ty {
            type Error = ();

            fn try_from(value: Size) -> Result<Self, Self::Error> {
                match value {
                    $pat => Ok(Self),
                    _ => Err(()),
                }
            }
        }
    };
}

try_from_size!(Q, Size::Qword);
try_from_size!(D, Size::Dword);
try_from_size!(W, Size::Word);
try_from_size!(B, Size::Byte);
