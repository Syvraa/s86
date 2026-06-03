use crate::operands::{
    immediates::Imm64,
    mem::Mem,
    operand::Operand,
    reg::Reg,
    sizeparams::{ImmSize, OpSize},
    sizes::QDWB,
};

#[derive(Clone, Copy)]
pub enum SimulatorOperand {
    Reg(Reg<QDWB>),
    Mem(Mem<QDWB>),
    Imm(Imm64),
}

impl<RS, MS, IS> From<Operand<RS, MS, IS>> for SimulatorOperand
where
    RS: OpSize,
    MS: OpSize,
    IS: ImmSize,
{
    fn from(value: Operand<RS, MS, IS>) -> Self {
        match value {
            Operand::Reg(reg) => SimulatorOperand::Reg(reg.into_all()),
            Operand::Mem(mem) => SimulatorOperand::Mem(mem.into_all()),
            Operand::Imm(imm) => SimulatorOperand::Imm(imm.into()),
        }
    }
}

impl<S> From<Reg<S>> for SimulatorOperand
where
    S: OpSize,
{
    fn from(value: Reg<S>) -> Self {
        SimulatorOperand::Reg(value.into_all())
    }
}

impl<S> From<Mem<S>> for SimulatorOperand
where
    S: OpSize,
{
    fn from(value: Mem<S>) -> Self {
        SimulatorOperand::Mem(value.into_all())
    }
}
