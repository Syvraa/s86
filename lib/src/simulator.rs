use crate::{
    instruction::Instr,
    label_parser::{LabelParser, fix_opcode_label_definitions},
    lexer::Lexer,
    operands::{Mem, RM, Reg, SimulatorOperand, Size},
    parser::Parser,
    registers::Registers,
};

/// Memory is stored in little endian format.
pub struct Simulator {
    pub registers: Registers,
    pub memory: Vec<u8>,
    instrs: Vec<Instr>,
    curr_instr: usize,
}

impl Simulator {
    #[must_use]
    pub fn new(source: &str, mem_size: usize) -> Self {
        let mut tokens = Lexer::new(source).lex();
        fix_opcode_label_definitions(&mut tokens);
        let labels = LabelParser::new(tokens.iter()).parse();
        let parsed = Parser::new(tokens.iter(), labels).parse();

        let memory = vec![0; mem_size];

        let rsp = if mem_size > 0 { mem_size - 1 } else { 0 } as u64;
        let registers = Registers {
            rsp,
            ..Registers::default()
        };

        Self {
            registers,
            memory,
            instrs: parsed,
            curr_instr: 0,
        }
    }

    pub fn run(&mut self) {
        while self.curr_instr < self.instrs.len() {
            self.step();
        }
    }

    pub fn step(&mut self) {
        if self.curr_instr >= self.instrs.len() {
            return;
        }

        // Needed, otherwise we would not execute the instruction we branched to.
        let mut branched = false;
        match &self.instrs[self.curr_instr] {
            Instr::Mov { dest, src } => self.do_mov(*dest, *src),
            Instr::MovMem { dest, src } => self.do_mov(*dest, *src),
            Instr::Add { dest, src } | Instr::Sub { dest, src } => {
                self.do_add_sub(*dest, *src);
            }
            Instr::AddMem { dest, src } | Instr::SubMem { dest, src } => {
                self.do_add_sub(*dest, *src);
            }
            Instr::Xor { dest, src } => {
                self.do_xor(*dest, *src);
            }
            Instr::XorMem { dest, src } => {
                self.do_xor(*dest, *src);
            }
            Instr::Jmp { dest }
            | Instr::Je { dest }
            | Instr::Jne { dest }
            | Instr::Ja { dest }
            | Instr::Jae { dest }
            | Instr::Jb { dest }
            | Instr::Jbe { dest }
            | Instr::Jg { dest }
            | Instr::Jge { dest }
            | Instr::Jl { dest }
            | Instr::Jle { dest } => {
                if self.should_branch(self.current_instr()) {
                    self.curr_instr = *dest;
                    branched = true;
                }
            }
            Instr::Cmp { dest, src } => self.do_cmp(*dest, *src),
            Instr::CmpMem { dest, src } => {
                self.do_cmp(*dest, *src);
            }
        }

        if !branched {
            self.curr_instr += 1;
        }
    }

    pub fn reset(&mut self) {
        self.registers = Registers::default();
        self.curr_instr = 0;
    }

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
    fn write_memory(&mut self, mem: Mem, value: u64) -> Result<(), ()> {
        let address = self.get_mem_index(mem);
        let bytes = value.to_le_bytes();
        match mem.size {
            Size::Byte => {
                self.memory
                    .get_mut(address..address + 1)
                    .ok_or(())?
                    .copy_from_slice(&bytes[0..1]);
            }
            Size::Word => self
                .memory
                .get_mut(address..address + 2)
                .ok_or(())?
                .copy_from_slice(&bytes[0..2]),
            Size::Dword => self
                .memory
                .get_mut(address..address + 4)
                .ok_or(())?
                .copy_from_slice(&bytes[0..4]),
            Size::Qword => self
                .memory
                .get_mut(address..address + 8)
                .ok_or(())?
                .copy_from_slice(&bytes[0..8]),
        }

        Ok(())
    }

    /// Reads from the given memory operand's index.
    /// Returns a `Ok(u64)` with the unread bits zeroed or `Err(())` if the address was out of
    /// bounds.
    fn read_memory(&self, mem: Mem) -> Result<u64, ()> {
        let address = self.get_mem_index(mem);
        let mut bytes = [0u8; 8];
        let source = match mem.size {
            Size::Byte => self.memory.get(address..address + 1).ok_or(())?,
            Size::Word => self.memory.get(address..address + 2).ok_or(())?,
            Size::Dword => self.memory.get(address..address + 4).ok_or(())?,
            Size::Qword => self.memory.get(address..address + 8).ok_or(())?,
        };
        bytes[0..source.len()].copy_from_slice(source);

        Ok(u64::from_le_bytes(bytes))
    }

    /// Writes to the given operand. Truncates `value` to the given size.
    fn write(&mut self, dest: impl Into<RM>, value: u64) -> Result<(), ()> {
        match dest.into() {
            RM::Reg(reg) => self.registers.write(reg, value),
            RM::Mem(mem) => self.write_memory(mem, value)?,
        }

        Ok(())
    }

    /// Gets the value of the operand. Returns `Err(())` if memory access was out of bounds.
    fn get_value(&self, src: impl Into<SimulatorOperand>) -> Result<u64, ()> {
        match src.into() {
            SimulatorOperand::Imm(imm) => Ok(imm),
            SimulatorOperand::Reg(reg) => Ok(self.registers.read(reg)),
            SimulatorOperand::Mem(mem) => self.read_memory(mem),
        }
    }

    fn do_mov(&mut self, dest: impl Into<RM>, src: impl Into<SimulatorOperand>) {
        let value = self
            .get_value(src)
            .expect("source memory access out of bounds");

        self.write(dest, value)
            .expect("destination memory access out of bounds");
    }

    fn do_add_sub<Dest>(&mut self, dest: Dest, src: impl Into<SimulatorOperand>)
    where
        Dest: Into<RM> + Copy,
    {
        let lhs = self
            .get_value(dest)
            .expect("destination memory access out of bounds");
        let rhs = self
            .get_value(src)
            .expect("source memory access out of bounds");

        let ((_, unsigned_overflow), (result, signed_overflow)) = match *self.current_instr() {
            Instr::Add { .. } => (
                lhs.overflowing_add(rhs),
                lhs.cast_signed().overflowing_add(rhs.cast_signed()),
            ),
            Instr::Sub { .. } => (
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
            .expect("destination memory access out of bounds");
    }

    fn do_xor<Dest>(&mut self, dest: Dest, src: impl Into<SimulatorOperand>)
    where
        Dest: Into<RM> + Copy,
    {
        let lhs = self
            .get_value(dest)
            .expect("destination memory access out of bounds");

        let rhs = self
            .get_value(src)
            .expect("destination memory access out of bounds");

        let result = lhs ^ rhs;

        self.registers.flags.set_cf(false);
        self.registers.flags.set_of(false);
        self.registers.flags.set_zf(result == 0);
        self.registers
            .flags
            .set_sf(result.cast_signed().signum() == -1);
        self.write(dest, result)
            .expect("destination memory access out of bounds");
    }

    fn do_cmp<Dest>(&mut self, dest: Dest, src: impl Into<SimulatorOperand>)
    where
        Dest: Into<RM> + Copy,
    {
        let lhs = self
            .get_value(dest)
            .expect("destination memory access out of bounds");
        let rhs = self
            .get_value(src)
            .expect("destination memory access out of bounds");

        let (_, unsigned_overflow) = lhs.overflowing_sub(rhs);
        let (result, signed_overflow) = lhs.cast_signed().overflowing_sub(rhs.cast_signed());
        self.registers.flags.set_cf(unsigned_overflow);
        self.registers.flags.set_of(signed_overflow);
        self.registers.flags.set_zf(result == 0);
        self.registers.flags.set_sf(result.signum() == -1);
    }

    fn should_branch(&self, op: &Instr) -> bool {
        match op {
            Instr::Jmp { dest: _ } => true,
            Instr::Je { dest: _ } => self.registers.flags.zf(),
            Instr::Jne { dest: _ } => !self.registers.flags.zf(),
            Instr::Ja { dest: _ } => !self.registers.flags.cf() && !self.registers.flags.zf(),
            Instr::Jae { dest: _ } => !self.registers.flags.cf(),
            Instr::Jb { dest: _ } => self.registers.flags.cf(),
            Instr::Jbe { dest: _ } => self.registers.flags.cf() || self.registers.flags.zf(),
            Instr::Jg { dest: _ } => {
                !self.registers.flags.zf() && self.registers.flags.sf() == self.registers.flags.of()
            }
            Instr::Jge { dest: _ } => self.registers.flags.sf() == self.registers.flags.of(),
            Instr::Jl { dest: _ } => self.registers.flags.sf() != self.registers.flags.of(),
            Instr::Jle { dest: _ } => {
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
