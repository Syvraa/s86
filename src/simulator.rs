use crate::{
    instruction::Instr,
    lexer::Lexer,
    operands::{Imm32, Imm64, Reg, RegOrImm32, RegOrImm64},
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
        let tokens = Lexer::new(source).lex();
        let instrs = Parser::new(tokens.iter()).parse();

        Self {
            registers: Registers::default(),
            instrs,
            curr_instr: 0,
        }
    }

    pub fn run(&mut self) {
        while self.curr_instr < self.instrs.len() {
            self.step();
        }
    }

    pub fn step(&mut self) {
        match self.instrs[self.curr_instr] {
            Instr::Mov { dest, src } => self.do_mov(dest, src),
            Instr::Add { dest, src } => self.do_add(dest, src),
            Instr::Sub { dest, src } => self.do_sub(dest, src),
            Instr::Xor { dest, src } => self.do_xor(dest, src),
            _ => todo!(),
        }
        self.curr_instr += 1;
    }

    pub fn reset(&mut self) {
        self.registers = Registers::default();
        self.curr_instr = 0;
    }

    fn do_mov(&mut self, dest: Reg, src: RegOrImm64) {
        match src {
            RegOrImm64::Imm(Imm64(val)) => self
                .registers
                .get_mut_reg(dest)
                .copy_from_slice(&val.to_ne_bytes()),
            // Otherwise we get complaints about multiple mutable borrows.
            RegOrImm64::Reg(reg) => unsafe {
                self.registers
                    .get_mut_reg(dest)
                    .as_mut_ptr()
                    .copy_from(self.registers.get_mut_reg(reg).as_mut_ptr(), 8);
            },
        }
    }

    fn do_add(&mut self, dest: Reg, src: RegOrImm32) {
        let lhs = u64::from_ne_bytes(*self.registers.get_mut_reg(dest));
        let rhs = match src {
            RegOrImm32::Imm(Imm32(val)) => u64::from(val),
            RegOrImm32::Reg(reg) => u64::from_ne_bytes(*self.registers.get_mut_reg(reg)),
        };

        let result = lhs + rhs;
        self.registers
            .get_mut_reg(dest)
            .copy_from_slice(&result.to_ne_bytes());
    }

    fn do_sub(&mut self, dest: Reg, src: RegOrImm32) {
        let lhs = u64::from_ne_bytes(*self.registers.get_mut_reg(dest));
        let rhs = match src {
            RegOrImm32::Imm(Imm32(val)) => u64::from(val),
            RegOrImm32::Reg(reg) => u64::from_ne_bytes(*self.registers.get_mut_reg(reg)),
        };

        let result = lhs - rhs;
        self.registers
            .get_mut_reg(dest)
            .copy_from_slice(&result.to_ne_bytes());
    }

    fn do_xor(&mut self, dest: Reg, src: RegOrImm32) {
        let lhs = u64::from_ne_bytes(*self.registers.get_mut_reg(dest));
        let rhs = match src {
            RegOrImm32::Imm(Imm32(val)) => u64::from(val),
            RegOrImm32::Reg(reg) => u64::from_ne_bytes(*self.registers.get_mut_reg(reg)),
        };

        let result = lhs ^ rhs;
        self.registers
            .get_mut_reg(dest)
            .copy_from_slice(&result.to_ne_bytes());
    }
}
