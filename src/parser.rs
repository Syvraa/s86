use std::slice::Iter;

use crate::{
    instruction::Instr,
    operands::{RegOrImm32, RegOrImm64},
    tokens::{Opcode, Token},
};

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    instrs: Vec<Instr>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Iter<'a, Token>) -> Self {
        Self {
            tokens,
            instrs: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Vec<Instr> {
        while let Some(token) = self.next() {
            match token {
                Token::Opcode(op) => self.parse_opcode(*op),
                _ => todo!(),
            }
        }

        self.instrs
    }

    fn parse_opcode(&mut self, op: Opcode) {
        type O = Opcode;
        match op {
            O::Mov => self.parse_mov(),
            O::Add | O::Sub | O::Xor => self.parse_binary_op(op),
            _ => todo!(),
        }
    }

    fn parse_mov(&mut self) {
        let Token::Reg(dest) = *self.consume() else {
            panic!("expected register");
        };

        let src = RegOrImm64::try_from(*self.consume()).expect("value is out of range");

        self.instrs.push(Instr::Mov { dest, src });
    }

    fn parse_binary_op(&mut self, op: Opcode) {
        type O = Opcode;
        let Token::Reg(dest) = *self.consume() else {
            panic!("expected register");
        };

        let src = RegOrImm32::try_from(*self.consume()).expect("value is out of range");

        // TODO: remove this
        #[allow(clippy::match_wildcard_for_single_variants)]
        let instr = match op {
            O::Add => Instr::Add { dest, src },
            O::Sub => Instr::Sub { dest, src },
            O::Xor => Instr::Xor { dest, src },
            _ => unreachable!("you forgot to add a case in parse_opcode"),
        };

        self.instrs.push(instr);
    }

    fn next(&mut self) -> Option<&'a Token> {
        self.tokens.next()
    }

    fn consume(&mut self) -> &Token {
        self.next().expect("premature end of input")
    }
}

#[allow(clippy::wildcard_imports)]
mod tests {
    use super::*;
    use crate::instruction::Instr;
    use crate::lexer::Lexer;
    use crate::operands::{Imm32, Reg};

    fn parse(source: &str) -> Vec<Instr> {
        let lexer = Lexer::new(source);
        let out = lexer.lex();
        let parser = Parser::new(out.iter());

        parser.parse()
    }

    #[test]
    fn single_mov() {
        let source = "mov rax, rbx";
        let parsed = parse(source);
        assert_eq!(
            parsed,
            vec![Instr::Mov {
                dest: Reg::Rax,
                src: RegOrImm64::Reg(Reg::Rbx)
            }]
        );
    }

    #[test]
    fn single_binary_op() {
        let source = "add rax, 8";
        let parsed = parse(source);
        assert_eq!(
            parsed,
            vec![Instr::Add {
                dest: Reg::Rax,
                src: RegOrImm32::Imm(Imm32(8))
            }]
        );
    }

    #[test]
    fn multiple_binary_ops() {
        let source = "
    add rax, 8
    xor rax, rax
    sub rbx, rax
";
        let parsed = parse(source);
        assert_eq!(
            parsed,
            vec![
                Instr::Add {
                    dest: Reg::Rax,
                    src: RegOrImm32::Imm(Imm32(8))
                },
                Instr::Xor {
                    dest: Reg::Rax,
                    src: RegOrImm32::Reg(Reg::Rax)
                },
                Instr::Sub {
                    dest: Reg::Rbx,
                    src: RegOrImm32::Reg(Reg::Rax)
                },
            ]
        );
    }

    #[test]
    #[should_panic(expected = "expected register")]
    fn invalid_operand() {
        let source = "add 8, rax";
        let _ = parse(source);
    }
}
