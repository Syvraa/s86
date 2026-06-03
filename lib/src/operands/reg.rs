use crate::operands::{
    mem::Size,
    registers::{BaseReg, ByteReg, DwordReg, IndexReg, QwordReg, WordReg},
    sizeparams::OpSize,
    sizes::{Q, QD, QDWB},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reg<S>
where
    S: OpSize,
{
    Qword(S::QRegT),
    Dword(S::DRegT),
    Word(S::WRegT),
    Byte(S::BRegT),
}

impl<RS1> Reg<RS1>
where
    RS1: OpSize,
{
    pub fn try_from_other<RS2>(value: &Reg<RS2>) -> Result<Self, ()>
    where
        RS2: OpSize,
        RS2::QRegT: TryInto<RS1::QRegT>,
        RS2::DRegT: TryInto<RS1::DRegT>,
        RS2::WRegT: TryInto<RS1::WRegT>,
        RS2::BRegT: TryInto<RS1::BRegT>,
    {
        match *value {
            Reg::Qword(reg) => Ok(Reg::Qword(reg.try_into().map_err(|_| ())?)),
            Reg::Dword(reg) => Ok(Reg::Dword(reg.try_into().map_err(|_| ())?)),
            Reg::Word(reg) => Ok(Reg::Word(reg.try_into().map_err(|_| ())?)),
            Reg::Byte(reg) => Ok(Reg::Byte(reg.try_into().map_err(|_| ())?)),
        }
    }

    pub fn try_into_other<RS2>(self) -> Result<Reg<RS2>, ()>
    where
        RS2: OpSize,
        RS1::QRegT: TryInto<RS2::QRegT>,
        RS1::DRegT: TryInto<RS2::DRegT>,
        RS1::WRegT: TryInto<RS2::WRegT>,
        RS1::BRegT: TryInto<RS2::BRegT>,
    {
        Reg::<RS2>::try_from_other(&self)
    }

    pub fn into_all(self) -> Reg<QDWB> {
        match self {
            Reg::Qword(reg) => Reg::Qword(reg.into()),
            Reg::Dword(reg) => Reg::Dword(reg.into()),
            Reg::Word(reg) => Reg::Word(reg.into()),
            Reg::Byte(reg) => Reg::Byte(reg.into()),
        }
    }
}

macro_rules! whole_reg_helper {
    ($variant:expr) => {
        Reg::Qword($variant.into())
    };
}

impl<S> Reg<S>
where
    S: OpSize,
{
    pub fn size(&self) -> Size {
        match self {
            Self::Qword(_) => Size::Qword,
            Self::Dword(_) => Size::Dword,
            Self::Word(_) => Size::Word,
            Self::Byte(_) => Size::Byte,
        }
    }

    pub fn to_whole_reg(&self) -> Reg<Q> {
        type Q = QwordReg;
        type D = DwordReg;
        type W = WordReg;
        type B = ByteReg;
        // .into() is fine here, since if the branch is hit, it's not !.
        match *self {
            Self::Qword(reg) => match reg.into() {
                Q::Rax => whole_reg_helper!(Q::Rax),
                Q::Rbx => whole_reg_helper!(Q::Rbx),
                Q::Rcx => whole_reg_helper!(Q::Rcx),
                Q::Rdx => whole_reg_helper!(Q::Rdx),
                Q::Rsi => whole_reg_helper!(Q::Rsi),
                Q::Rdi => whole_reg_helper!(Q::Rdi),
                Q::Rsp => whole_reg_helper!(Q::Rsp),
                Q::Rbp => whole_reg_helper!(Q::Rbp),
                Q::R8 => whole_reg_helper!(Q::R8),
                Q::R9 => whole_reg_helper!(Q::R9),
                Q::R10 => whole_reg_helper!(Q::R10),
                Q::R11 => whole_reg_helper!(Q::R11),
                Q::R12 => whole_reg_helper!(Q::R12),
                Q::R13 => whole_reg_helper!(Q::R13),
                Q::R14 => whole_reg_helper!(Q::R14),
                Q::R15 => whole_reg_helper!(Q::R15),
            },
            Self::Dword(reg) => match reg.into() {
                D::Eax => whole_reg_helper!(Q::Rax),
                D::Ebx => whole_reg_helper!(Q::Rbx),
                D::Ecx => whole_reg_helper!(Q::Rcx),
                D::Edx => whole_reg_helper!(Q::Rdx),
                D::Esi => whole_reg_helper!(Q::Rsi),
                D::Edi => whole_reg_helper!(Q::Rdi),
                D::Esp => whole_reg_helper!(Q::Rsp),
                D::Ebp => whole_reg_helper!(Q::Rbp),
                D::R8d => whole_reg_helper!(Q::R8),
                D::R9d => whole_reg_helper!(Q::R9),
                D::R10d => whole_reg_helper!(Q::R10),
                D::R11d => whole_reg_helper!(Q::R11),
                D::R12d => whole_reg_helper!(Q::R12),
                D::R13d => whole_reg_helper!(Q::R13),
                D::R14d => whole_reg_helper!(Q::R14),
                D::R15d => whole_reg_helper!(Q::R15),
            },
            Self::Word(reg) => match reg.into() {
                W::Ax => whole_reg_helper!(Q::Rax),
                W::Bx => whole_reg_helper!(Q::Rbx),
                W::Cx => whole_reg_helper!(Q::Rcx),
                W::Dx => whole_reg_helper!(Q::Rdx),
                W::Si => whole_reg_helper!(Q::Rsi),
                W::Di => whole_reg_helper!(Q::Rdi),
                W::Sp => whole_reg_helper!(Q::Rsp),
                W::Bp => whole_reg_helper!(Q::Rbp),
                W::R8w => whole_reg_helper!(Q::R8),
                W::R9w => whole_reg_helper!(Q::R9),
                W::R10w => whole_reg_helper!(Q::R10),
                W::R11w => whole_reg_helper!(Q::R11),
                W::R12w => whole_reg_helper!(Q::R12),
                W::R13w => whole_reg_helper!(Q::R13),
                W::R14w => whole_reg_helper!(Q::R14),
                W::R15w => whole_reg_helper!(Q::R15),
            },
            Self::Byte(reg) => match reg.into() {
                B::Ah | B::Al => whole_reg_helper!(Q::Rax),
                B::Bh | B::Bl => whole_reg_helper!(Q::Rbx),
                B::Ch | B::Cl => whole_reg_helper!(Q::Rcx),
                B::Dh | B::Dl => whole_reg_helper!(Q::Rdx),
                B::Sil => whole_reg_helper!(Q::Rsi),
                B::Dil => whole_reg_helper!(Q::Rdi),
                B::Spl => whole_reg_helper!(Q::Rsp),
                B::Bpl => whole_reg_helper!(Q::Rbp),
                B::R8b => whole_reg_helper!(Q::R8),
                B::R9b => whole_reg_helper!(Q::R9),
                B::R10b => whole_reg_helper!(Q::R10),
                B::R11b => whole_reg_helper!(Q::R11),
                B::R12b => whole_reg_helper!(Q::R12),
                B::R13b => whole_reg_helper!(Q::R13),
                B::R14b => whole_reg_helper!(Q::R14),
                B::R15b => whole_reg_helper!(Q::R15),
            },
        }
    }
}

impl From<IndexReg> for Reg<QDWB> {
    fn from(value: IndexReg) -> Self {
        match value {
            IndexReg::Qword(reg) => Reg::Qword(QwordReg::from(reg)),
            IndexReg::Dword(reg) => Reg::Dword(DwordReg::from(reg)),
        }
    }
}

impl From<BaseReg> for Reg<QD> {
    fn from(value: BaseReg) -> Self {
        match value {
            BaseReg::Qword(reg) => Self::Qword(reg),
            BaseReg::Dword(reg) => Self::Dword(reg),
        }
    }
}
