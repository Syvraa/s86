use crate::operands::{
    mem::{Mem, Size},
    reg::Reg,
    sizeparams::{ImmSize, OpSize},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operand<RegSize, MemSize, ImmSize>
where
    RegSize: OpSize,
    MemSize: OpSize,
    ImmSize: crate::operands::sizeparams::ImmSize,
{
    Reg(Reg<RegSize>),
    Mem(Mem<MemSize>),
    Imm(ImmSize::ImmT),
}

impl<RS1, MS1, IS1> Operand<RS1, MS1, IS1>
where
    RS1: OpSize,
    MS1: OpSize,
    IS1: ImmSize,
{
    pub fn try_from_other<RS2, MS2, IS2>(
        value: Operand<RS2, MS2, IS2>,
    ) -> Result<Self, OperandConversionError>
    where
        RS2: OpSize,
        MS2: OpSize,
        IS2: ImmSize,
        RS2::QRegT: TryInto<RS1::QRegT>,
        RS2::DRegT: TryInto<RS1::DRegT>,
        RS2::WRegT: TryInto<RS1::WRegT>,
        RS2::BRegT: TryInto<RS1::BRegT>,
        IS2::ImmT: TryInto<IS1::ImmT>,
        OperandConversionError: From<<IS2::ImmT as TryInto<IS1::ImmT>>::Error>,
    {
        match value {
            Operand::Reg(reg) => Ok(Operand::Reg(
                reg.try_into_other()
                    .map_err(|()| OperandConversionError::WrongOperand)?,
            )),
            Operand::Mem(mem) => Ok(Operand::Mem(
                mem.try_into_other()
                    .map_err(|()| OperandConversionError::WrongOperand)?,
            )),
            Operand::Imm(imm) => Ok(Operand::Imm(imm.try_into()?)),
        }
    }
}

impl<RS, MS> Operand<RS, MS, !>
where
    RS: OpSize,
    MS: OpSize,
{
    pub fn size(&self) -> Size {
        match self {
            Operand::Reg(reg) => reg.size(),
            Operand::Mem(mem) => mem.size,
            // Why doesn't the compiler know that it is not possible?
            Operand::Imm(_) => unreachable!(),
        }
    }
}

/// Contains `Ok(Some(Operand))` if parsing was successful.
/// Contains `Ok(None)` if the parsed token was not a valid `Operand` (or there was no token
/// remaining).
/// Contains `ParsingError` if an error occured during parsing (for example, a memory operand or a
/// negative number could not be parsed). In that case, the error was already pushed to `self.errors`,
/// so you should just use `?` to return `None`.
// I know this is a really weird type but this was the most convenient way I could come up for
// having 2 error states while making the api convenient (just use ? if there is an error).
pub enum OperandParseResult<RS, MS, IS>
where
    RS: OpSize,
    MS: OpSize,
    IS: ImmSize,
{
    Ok(Option<Operand<RS, MS, IS>>),
    ParsingError,
}

impl<RS, MS, IS> std::ops::FromResidual for OperandParseResult<RS, MS, IS>
where
    RS: OpSize,
    MS: OpSize,
    IS: ImmSize,
{
    fn from_residual(_: <Self as std::ops::Try>::Residual) -> Self {
        Self::ParsingError
    }
}

impl<T> std::ops::FromResidual<ParsingError> for Option<T> {
    fn from_residual(_: ParsingError) -> Self {
        None
    }
}

pub struct ParsingError;
impl<RS, MS, IS> std::ops::Residual<Option<Operand<RS, MS, IS>>> for ParsingError
where
    RS: OpSize,
    MS: OpSize,
    IS: ImmSize,
{
    type TryType = OperandParseResult<RS, MS, IS>;
}

impl<RS, MS, IS> std::ops::Try for OperandParseResult<RS, MS, IS>
where
    RS: OpSize,
    MS: OpSize,
    IS: ImmSize,
{
    type Output = Option<Operand<RS, MS, IS>>;
    type Residual = ParsingError;

    fn from_output(output: Self::Output) -> Self {
        Self::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Ok(value) => std::ops::ControlFlow::Continue(value),
            Self::ParsingError => std::ops::ControlFlow::Break(ParsingError),
        }
    }
}

#[derive(Clone, Copy)]
pub enum OperandConversionError {
    WrongOperand,
    ImmediateOutOfRangeForDword,
}

impl From<std::convert::Infallible> for OperandConversionError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
