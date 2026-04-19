#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    diff::{Diff, DiffReg, MemDiff, RegDiff, StateDiff},
    instruction::{Instr, InstrKind},
    label_parser::{LabelParser, fix_opcode_label_definitions},
    lexer::Lexer,
    operands::{Mem, RM, Reg, SimulatorOperand, Size},
    parser::Parser,
    registers::Registers,
    simulator_error::SimulatorError,
    syntax_error::SyntaxError,
};

/// Memory is stored in little endian format.
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
pub struct Simulator {
    pub registers: Registers,
    #[cfg(not(feature = "wasm-bindgen"))]
    pub memory: Vec<u8>,
    #[cfg(feature = "wasm-bindgen")]
    memory: Vec<u8>,
    instrs: Vec<Instr>,
    curr_instr: usize,
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
impl Simulator {
    /// Keep in mind this clones the entire array, so probably don't call it frequently.
    #[allow(clippy::must_use_candidate)]
    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
    #[cfg(feature = "wasm-bindgen")]
    pub fn memory(&self) -> Box<[u8]> {
        self.memory.clone().into_boxed_slice()
    }

    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(constructor))]
    /// Creates a new Simulator instance from the specified string of instructions and with the
    /// specified amount of memory (in bytes).
    ///
    /// # Errors
    /// Returns `Err(Vec<SyntaxError>)` with all the errors that occured during lexing/parsing.
    pub fn new(source: &str, mem_size: usize) -> Result<Simulator, Vec<SyntaxError>> {
        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

        let mut tokens = Lexer::new(source).lex()?;
        fix_opcode_label_definitions(&mut tokens);
        let labels = LabelParser::new(tokens.iter()).parse()?;
        let parsed = Parser::new(tokens.iter(), labels).parse()?;

        let memory = vec![0; mem_size];

        let rsp = if mem_size > 0 { mem_size - 1 } else { 0 } as u64;
        let registers = Registers {
            rsp,
            ..Registers::default()
        };

        Ok(Self {
            registers,
            memory,
            instrs: parsed,
            curr_instr: 0,
        })
    }

    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
    #[allow(clippy::missing_errors_doc)]
    pub fn run(&mut self) -> Result<(), SimulatorError> {
        while self.curr_instr < self.instrs.len() {
            // We won't get an Err(SimulatorError::EndOfInstruction), because we just checked the
            // condition.
            self.step()?;
        }

        Ok(())
    }

    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
    #[allow(clippy::missing_errors_doc)]
    pub fn step(&mut self) -> Result<StateDiff, SimulatorError> {
        if self.curr_instr >= self.instrs.len() {
            return Err(SimulatorError::EndOfInstructions);
        }

        // Needed, otherwise we would not execute the instruction we branched to.
        let mut branched = false;
        let mut diffs = StateDiff::default();
        match &self.instrs[self.curr_instr].kind {
            InstrKind::Mov { dest, src } => diffs.push(self.do_mov(*dest, *src)?),
            InstrKind::MovMem { dest, src } => diffs.push(self.do_mov(*dest, *src)?),
            InstrKind::Add { dest, src } | InstrKind::Sub { dest, src } => {
                diffs.push(self.do_add_sub(*dest, *src)?);
            }
            InstrKind::AddMem { dest, src } | InstrKind::SubMem { dest, src } => {
                diffs.push(self.do_add_sub(*dest, *src)?);
            }
            InstrKind::Xor { dest, src } => diffs.push(self.do_xor(*dest, *src)?),
            InstrKind::XorMem { dest, src } => diffs.push(self.do_xor(*dest, *src)?),
            InstrKind::Jmp { dest }
            | InstrKind::Je { dest }
            | InstrKind::Jne { dest }
            | InstrKind::Ja { dest }
            | InstrKind::Jae { dest }
            | InstrKind::Jb { dest }
            | InstrKind::Jbe { dest }
            | InstrKind::Jg { dest }
            | InstrKind::Jge { dest }
            | InstrKind::Jl { dest }
            | InstrKind::Jle { dest } => {
                if self.should_branch(&self.current_instr().kind) {
                    self.curr_instr = *dest;
                    branched = true;
                }
            }
            InstrKind::Cmp { dest, src } => diffs.push(self.do_cmp(*dest, *src)?),
            InstrKind::CmpMem { dest, src } => diffs.push(self.do_cmp(*dest, *src)?),
        }

        if !branched {
            self.curr_instr += 1;
        }

        Ok(diffs)
    }

    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
    pub fn reset(&mut self) {
        self.registers = Registers::default();
        self.curr_instr = 0;
    }

    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
    #[must_use]
    pub fn current_line(&self) -> Option<usize> {
        if self.curr_instr >= self.instrs.len() {
            None
        } else {
            Some(self.current_instr().line)
        }
    }
}

impl Simulator {
    #[allow(clippy::cast_possible_truncation)]
    /// Gets the index the memory operand points to.
    /// Basically `lea`.
    fn get_mem_index(&self, mem: Mem) -> usize {
        let mut address = mem
            .base
            .map_or(0, |base_reg| self.registers.read(Reg::from(base_reg)));
        address += mem.index.map_or(0, |idx| {
            self.registers.read(Reg::from(idx.index)) * idx.scale as u64
        });
        address += mem.disp.map_or(0, u64::from);

        address as usize
    }

    /// Writes to the given memory operand's index.
    /// Returns `Err(())` if the address was out of bounds.
    fn write_memory(&mut self, mem: Mem, value: u64) -> Result<MemDiff, SimulatorError> {
        let address = self.get_mem_index(mem);
        let bytes = value.to_le_bytes();
        match mem.size {
            Size::Byte => self
                .memory
                .get_mut(address..address + 1)
                .ok_or(SimulatorError::InvalidMemAccess)?
                .copy_from_slice(&bytes[0..1]),
            Size::Word => self
                .memory
                .get_mut(address..address + 2)
                .ok_or(SimulatorError::InvalidMemAccess)?
                .copy_from_slice(&bytes[0..2]),
            Size::Dword => self
                .memory
                .get_mut(address..address + 4)
                .ok_or(SimulatorError::InvalidMemAccess)?
                .copy_from_slice(&bytes[0..4]),
            Size::Qword => self
                .memory
                .get_mut(address..address + 8)
                .ok_or(SimulatorError::InvalidMemAccess)?
                .copy_from_slice(&bytes[0..8]),
        }

        Ok(MemDiff {
            address,
            size: mem.size,
            value,
        })
    }

    /// Reads from the given memory operand's index.
    /// Returns a `Ok(u64)` with the unread bits zeroed or `Err(())` if the address was out of
    /// bounds.
    fn read_memory(&self, mem: Mem) -> Result<u64, SimulatorError> {
        let address = self.get_mem_index(mem);
        let mut bytes = [0u8; 8];
        let source = match mem.size {
            Size::Byte => self
                .memory
                .get(address..address + 1)
                .ok_or(SimulatorError::InvalidMemAccess)?,
            Size::Word => self
                .memory
                .get(address..address + 2)
                .ok_or(SimulatorError::InvalidMemAccess)?,
            Size::Dword => self
                .memory
                .get(address..address + 4)
                .ok_or(SimulatorError::InvalidMemAccess)?,
            Size::Qword => self
                .memory
                .get(address..address + 8)
                .ok_or(SimulatorError::InvalidMemAccess)?,
        };
        bytes[0..source.len()].copy_from_slice(source);

        Ok(u64::from_le_bytes(bytes))
    }

    /// Writes to the given operand. Truncates `value` to the given size.
    fn write(&mut self, dest: impl Into<RM>, value: u64) -> Result<Diff, SimulatorError> {
        let diff;
        match dest.into() {
            RM::Reg(reg) => {
                diff = Diff::Reg(RegDiff {
                    reg: reg.into(),
                    value,
                });
                self.registers.write(reg, value);
            }
            RM::Mem(mem) => {
                diff = Diff::Mem(self.write_memory(mem, value)?);
            }
        }

        Ok(diff)
    }

    /// Gets the value of the operand. Returns `Err(())` if memory access was out of bounds.
    fn get_value(&self, src: impl Into<SimulatorOperand>) -> Result<u64, SimulatorError> {
        match src.into() {
            SimulatorOperand::Imm(imm) => Ok(imm),
            SimulatorOperand::Reg(reg) => Ok(self.registers.read(reg)),
            SimulatorOperand::Mem(mem) => self.read_memory(mem),
        }
    }

    fn do_mov(
        &mut self,
        dest: impl Into<RM>,
        src: impl Into<SimulatorOperand>,
    ) -> Result<Diff, SimulatorError> {
        let value = self.get_value(src)?;

        self.write(dest, value)
    }

    fn do_add_sub<Dest>(
        &mut self,
        dest: Dest,
        src: impl Into<SimulatorOperand>,
    ) -> Result<Diff, SimulatorError>
    where
        Dest: Into<RM> + Copy,
    {
        let lhs = self.get_value(dest)?;
        let rhs = self.get_value(src)?;

        let ((_, unsigned_overflow), (result, signed_overflow)) = match self.current_instr().kind {
            InstrKind::Add { .. } => (
                lhs.overflowing_add(rhs),
                lhs.cast_signed().overflowing_add(rhs.cast_signed()),
            ),
            InstrKind::Sub { .. } => (
                lhs.overflowing_sub(rhs),
                lhs.cast_signed().overflowing_sub(rhs.cast_signed()),
            ),
            _ => unreachable!("if you got this, you forgot to add a case"),
        };
        self.registers.flags.set_cf(unsigned_overflow);
        self.registers.flags.set_of(signed_overflow);
        self.registers.flags.set_zf(result == 0);
        self.registers.flags.set_sf(result.signum() == -1);

        self.write(dest, result.cast_unsigned())
    }

    fn do_xor<Dest>(
        &mut self,
        dest: Dest,
        src: impl Into<SimulatorOperand>,
    ) -> Result<Diff, SimulatorError>
    where
        Dest: Into<RM> + Copy,
    {
        let lhs = self.get_value(dest)?;

        let rhs = self.get_value(src)?;

        let result = lhs ^ rhs;

        self.registers.flags.set_cf(false);
        self.registers.flags.set_of(false);
        self.registers.flags.set_zf(result == 0);
        self.registers
            .flags
            .set_sf(result.cast_signed().signum() == -1);

        self.write(dest, result)
    }

    fn do_cmp<Dest>(
        &mut self,
        dest: Dest,
        src: impl Into<SimulatorOperand>,
    ) -> Result<Diff, SimulatorError>
    where
        Dest: Into<RM> + Copy,
    {
        let lhs = self.get_value(dest)?;
        let rhs = self.get_value(src)?;

        let (_, unsigned_overflow) = lhs.overflowing_sub(rhs);
        let (result, signed_overflow) = lhs.cast_signed().overflowing_sub(rhs.cast_signed());
        self.registers.flags.set_cf(unsigned_overflow);
        self.registers.flags.set_of(signed_overflow);
        self.registers.flags.set_zf(result == 0);
        self.registers.flags.set_sf(result.signum() == -1);

        Ok(Diff::Reg(RegDiff {
            reg: DiffReg::Flags,
            value: self.registers.flags.0,
        }))
    }

    fn should_branch(&self, op: &InstrKind) -> bool {
        match op {
            InstrKind::Jmp { dest: _ } => true,
            InstrKind::Je { dest: _ } => self.registers.flags.zf(),
            InstrKind::Jne { dest: _ } => !self.registers.flags.zf(),
            InstrKind::Ja { dest: _ } => !self.registers.flags.cf() && !self.registers.flags.zf(),
            InstrKind::Jae { dest: _ } => !self.registers.flags.cf(),
            InstrKind::Jb { dest: _ } => self.registers.flags.cf(),
            InstrKind::Jbe { dest: _ } => self.registers.flags.cf() || self.registers.flags.zf(),
            InstrKind::Jg { dest: _ } => {
                !self.registers.flags.zf() && self.registers.flags.sf() == self.registers.flags.of()
            }
            InstrKind::Jge { dest: _ } => self.registers.flags.sf() == self.registers.flags.of(),
            InstrKind::Jl { dest: _ } => self.registers.flags.sf() != self.registers.flags.of(),
            InstrKind::Jle { dest: _ } => {
                self.registers.flags.zf() && self.registers.flags.sf() != self.registers.flags.of()
            }
            _ => false,
        }
    }

    #[must_use]
    pub fn current_instr(&self) -> &'_ Instr {
        &self.instrs[self.curr_instr]
    }
}
