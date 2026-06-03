use crate::operands::{reg::Reg, sizeparams::OpSize};

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

impl<S> TryFrom<Reg<S>> for IndexReg
where
    S: OpSize,
{
    type Error = ();

    fn try_from(value: Reg<S>) -> Result<Self, Self::Error> {
        match value {
            Reg::Qword(reg) => Ok(Self::Qword(reg.try_into().map_err(|_| ())?)),
            Reg::Dword(reg) => Ok(Self::Dword(reg.try_into().map_err(|_| ())?)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaseReg {
    Qword(QwordReg),
    Dword(DwordReg),
}

impl<S> TryFrom<Reg<S>> for BaseReg
where
    S: OpSize,
{
    type Error = ();

    fn try_from(value: Reg<S>) -> Result<Self, Self::Error> {
        match value {
            // We can use .into() here, since if the branch is hit, that means the type of reg is
            // not !.
            Reg::Qword(reg) => Ok(Self::Qword(reg.into())),
            Reg::Dword(reg) => Ok(Self::Dword(reg.into())),
            _ => Err(()),
        }
    }
}
