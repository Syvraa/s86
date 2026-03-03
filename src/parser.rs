use std::{collections::HashMap, iter::Peekable, slice::Iter};

use crate::{
    instruction::Instr,
    operands::{Label, RegOrImm32, RegOrImm64},
    tokens::{Opcode, Token},
};

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    instrs: Vec<Instr>,
    labels: HashMap<Label, usize>,
    parent_label: Label,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Iter<'a, Token>, labels: HashMap<Label, usize>) -> Self {
        Self {
            tokens: tokens.peekable(),
            instrs: Vec::new(),
            labels,
            parent_label: Label(String::new()),
        }
    }

    pub fn parse(mut self) -> Vec<Instr> {
        while let Some(token) = self.next() {
            match token {
                Token::Opcode(op) => {
                    self.parse_opcode(*op);
                }
                Token::Label(label) => {
                    self.parent_label.clone_from(label);
                    // Consume the ":".
                    self.next();
                }
                Token::Sublabel(_) => {
                    // Consume the ":".
                    self.next();
                }
                _ => panic!("unexpected token"),
            }
        }

        self.instrs
    }

    fn parse_opcode(&mut self, op: Opcode) {
        type O = Opcode;
        match op {
            O::Mov => self.parse_mov(),
            O::Add | O::Sub | O::Xor => self.parse_binary_op(op),
            O::Jmp => self.parse_jmp(),
        }
    }

    fn parse_jmp(&mut self) {
        let label = match self.next().expect("expected label name") {
            Token::Opcode(op) => op.as_str(),
            Token::Label(Label(name)) => name,
            Token::Sublabel(Label(name)) => &(self.parent_label.0.clone() + name),
            _ => panic!("not a valid label name"),
        };

        let Some(instr_idx) = self.labels.get(&Label(label.to_string())).copied() else {
            panic!("no label {label} found");
        };

        self.instrs.push(Instr::Jmp { dest: instr_idx });
    }

    fn parse_mov(&mut self) {
        let Token::Reg(dest) = *self.consume() else {
            panic!("expected register");
        };
        assert_eq!(self.next(), Some(&Token::Comma), "expected comma");

        let src = RegOrImm64::try_from(self.consume()).expect("value is out of range");

        self.instrs.push(Instr::Mov { dest, src });
    }

    fn parse_binary_op(&mut self, op: Opcode) {
        type O = Opcode;
        let Token::Reg(dest) = *self.consume() else {
            panic!("expected register");
        };
        assert_eq!(self.next(), Some(&Token::Comma), "expected comma");

        let src = RegOrImm32::try_from(self.consume()).expect("value is out of range");

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instr;
    use crate::label_parser::{LabelParser, fix_opcode_label_definitions};
    use crate::lexer::Lexer;
    use crate::operands::{Imm32, Reg};

    struct ParseResult {
        instrs: Vec<Instr>,
        labels: HashMap<Label, usize>,
    }

    fn parse(source: &str) -> ParseResult {
        let lexer = Lexer::new(source);
        let mut tokens = lexer.lex();
        fix_opcode_label_definitions(&mut tokens);
        let labels = LabelParser::new(tokens.iter()).parse();
        let parser = Parser::new(tokens.iter(), labels.clone());
        let instrs = parser.parse();

        ParseResult { instrs, labels }
    }

    #[test]
    fn single_mov() {
        let source = "mov rax, rbx";
        let parsed = parse(source).instrs;
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
        let parsed = parse(source).instrs;
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
        let parsed = parse(source).instrs;
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
        let _ = parse(source).instrs;
    }

    #[test]
    fn jmp() {
        let source = "
    label:
        jmp label
";

        let parsed = parse(source);
        assert_eq!(parsed.instrs, vec![Instr::Jmp { dest: 0 }]);
        let mut expected_labels = HashMap::new();
        expected_labels.insert(Label("label".into()), 0);
        assert_eq!(parsed.labels, expected_labels);
    }

    #[test]
    #[should_panic(expected = "no label label found")]
    fn jmp_not_exists() {
        let source = "
    jmp label
";

        let _ = parse(source);
    }

    #[test]
    #[should_panic(expected = "not a valid label name")]
    fn jmp_invalid_operand() {
        let source = "
    jmp 2dfa
";

        let _ = parse(source);
    }
}
