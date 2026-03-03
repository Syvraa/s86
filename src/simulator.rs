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
            Instr::Add { dest, src } | Instr::Sub { dest, src } | Instr::Xor { dest, src } => {
                self.do_binary_op(*dest, *src);
            }
            Instr::Jmp { dest } => {
                self.curr_instr = *dest;
                branched = true;
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

    fn do_binary_op(&mut self, dest: Reg, src: RegOrImm32) {
        let lhs = u64::from_ne_bytes(*self.registers.get_mut_reg(dest));
        let rhs = match src {
            RegOrImm32::Imm(imm) => u64::from(imm),
            RegOrImm32::Reg(reg) => u64::from_ne_bytes(*self.registers.get_mut_reg(reg)),
        };

        let result = match *self.current_instr() {
            Instr::Add { .. } => lhs.overflowing_add(rhs).0,
            Instr::Sub { .. } => lhs.overflowing_sub(rhs).0,
            Instr::Xor { .. } => lhs ^ rhs,
            _ => unreachable!("if you got this, you forgot to add a case"),
        };
        self.registers
            .get_mut_reg(dest)
            .copy_from_slice(&result.to_ne_bytes());
    }

    #[must_use]
    pub fn current_instr(&self) -> &'_ Instr {
        &self.instrs[self.curr_instr]
    }
}
