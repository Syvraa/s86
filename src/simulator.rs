use crate::{
    instruction::Instr,
    label_parser::{LabelParser, fix_opcode_label_definitions},
    lexer::Lexer,
    operands::{Imm64, Reg, RegOrImm32, RegOrImm64},
    parser::Parser,
    registers::Registers,
};

pub struct Simulator {
    pub registers: Registers,
    instrs: Vec<Instr>,
    curr_instr: usize,
}

impl Simulator {
    #[must_use]
    pub fn new(source: &str) -> Self {
        let mut tokens = Lexer::new(source).lex();
        fix_opcode_label_definitions(&mut tokens);
        let labels = LabelParser::new(tokens.iter()).parse();
        let parsed = Parser::new(tokens.iter(), labels).parse();

        Self {
            registers: Registers::default(),
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
            Instr::Add { dest, src } | Instr::Sub { dest, src } => {
                self.do_add_sub(*dest, *src);
            }
            Instr::Xor { dest, src } => {
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
        }

        if !branched {
            self.curr_instr += 1;
        }
    }

    pub fn reset(&mut self) {
        self.registers = Registers::default();
        self.curr_instr = 0;
    }

    fn do_mov(&mut self, dest: Reg, src: RegOrImm64) {
        match src {
            RegOrImm64::Imm(Imm64(val)) => *self.registers.get_mut_reg(dest) = val,
            RegOrImm64::Reg(reg) => {
                *self.registers.get_mut_reg(dest) = *self.registers.get_mut_reg(reg);
            }
        }
    }

    fn do_add_sub(&mut self, dest: Reg, src: RegOrImm32) {
        let lhs = *self.registers.get_mut_reg(dest);
        let rhs = match src {
            RegOrImm32::Imm(imm) => u64::from(imm),
            RegOrImm32::Reg(reg) => *self.registers.get_mut_reg(reg),
        };

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
        *self.registers.get_mut_reg(dest) = result.cast_unsigned();
    }

    fn do_xor(&mut self, dest: Reg, src: RegOrImm32) {
        let lhs = *self.registers.get_mut_reg(dest);
        let rhs = match src {
            RegOrImm32::Imm(imm) => u64::from(imm),
            RegOrImm32::Reg(reg) => *self.registers.get_mut_reg(reg),
        };

        let result = lhs ^ rhs;

        self.registers.flags.set_cf(false);
        self.registers.flags.set_of(false);
        self.registers.flags.set_zf(result == 0);
        self.registers
            .flags
            .set_sf(result.cast_signed().signum() == -1);
        *self.registers.get_mut_reg(dest) = result;
    }

    fn do_cmp(&mut self, dest: Reg, src: RegOrImm32) {
        let lhs = *self.registers.get_mut_reg(dest);
        let rhs = match src {
            RegOrImm32::Imm(imm) => u64::from(imm),
            RegOrImm32::Reg(reg) => *self.registers.get_mut_reg(reg),
        };

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
