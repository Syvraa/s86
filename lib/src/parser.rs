use std::{collections::HashMap, iter::Peekable, slice::Iter};

use crate::{
    instruction::Instr,
    operands::{
        BaseReg, Bits as _, Imm32, Index, IndexReg, Label, Mem, Operand, RI32, RIConversionError,
        RM, RMI32, RMI64, Scale, Size,
    },
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
            O::Add | O::Sub | O::Xor | O::Cmp => self.parse_binary_op(op),
            O::Jmp => self.parse_jmp(),
            O::Je | O::Jz => {
                let dest = self.get_instr_idx();
                self.instrs.push(Instr::Je { dest });
            }
            O::Jne | O::Jnz => {
                let dest = self.get_instr_idx();
                self.instrs.push(Instr::Jne { dest });
            }
            O::Ja | O::Jnbe => {
                let dest = self.get_instr_idx();
                self.instrs.push(Instr::Ja { dest });
            }
            O::Jae | O::Jnb => {
                let dest = self.get_instr_idx();
                self.instrs.push(Instr::Jae { dest });
            }
            O::Jb | O::Jnae => {
                let dest = self.get_instr_idx();
                self.instrs.push(Instr::Jb { dest });
            }
            O::Jbe | O::Jna => {
                let dest = self.get_instr_idx();
                self.instrs.push(Instr::Jbe { dest });
            }
            O::Jg | O::Jnle => {
                let dest = self.get_instr_idx();
                self.instrs.push(Instr::Jg { dest });
            }
            O::Jge | O::Jnl => {
                let dest = self.get_instr_idx();
                self.instrs.push(Instr::Jge { dest });
            }
            O::Jl | O::Jnge => {
                let dest = self.get_instr_idx();
                self.instrs.push(Instr::Jl { dest });
            }
            O::Jle | O::Jng => {
                let dest = self.get_instr_idx();
                self.instrs.push(Instr::Jle { dest });
            }
        }
    }

    // TODO: Rewrite this. I hate it. There's definitely a better way.
    fn parse_memory(&mut self, size: Size) -> Mem {
        assert!(
            self.next() == Some(&Token::LBracket),
            "expected memory operand"
        );
        let mut base = None;
        let mut index = None;
        let mut disp = None;

        let mut positive = true;
        while let Some(tok) = self.next() {
            match tok {
                Token::RBracket => {
                    if base.is_none() && index.is_none() && disp.is_none() {
                        panic!("incomplete memory operand");
                    } else {
                        return Mem {
                            base,
                            index,
                            disp,
                            size,
                        };
                    }
                }
                // If we encounter a - or a +, then we start over, as we would fail the check at
                // the end if it was something like + rax (we would consume the + and panic because
                // the next token is not a +, a - or a ]).
                Token::Minus => {
                    positive = !positive;
                    continue;
                }
                Token::Plus => {
                    continue;
                }
                Token::Reg(reg)
                    if *self.peek().expect("premature end of token stream") != Token::Star =>
                {
                    if base.is_none() {
                        base = Some(BaseReg::try_from(*reg).expect("invalid register for base"));
                    } else {
                        panic!("base register already given");
                    }
                }
                Token::Reg(reg)
                    if *self.peek().expect("premature end of token stream") == Token::Star =>
                {
                    assert!(index.is_none(), "index already given");

                    self.next();
                    if let Some(Token::Number(imm)) = self.next() {
                        index = Some(Index {
                            index: IndexReg::try_from(*reg)
                                .expect("rsp cannot be used for indexing"),
                            scale: Scale::try_from(*imm)
                                .expect("scale factor can only be 1, 2, 4 or 8"),
                        });
                    } else {
                        panic!("expected scale factor after index register");
                    }
                }
                Token::Number(val) => {
                    let (new_disp, overflow);
                    if let Some(curr_disp) = disp {
                        (new_disp, overflow) = curr_disp.0.cast_signed().overflowing_add(
                            i32::try_from(*val).expect("displacement out of range"),
                        );
                    } else {
                        (new_disp, overflow) = 0_i32.overflowing_add(
                            i32::try_from(*val).expect("displacement out of range"),
                        );
                    }

                    assert!(!overflow, "displacement exceeds signed dword bounds");
                    disp = Some(Imm32(new_disp.cast_unsigned()));
                }
                _ => panic!("unexpected token"),
            }

            let peeked = self.peek().expect("premature end of token stream");
            assert!(
                !(*peeked != Token::Plus && *peeked != Token::Minus && *peeked != Token::RBracket),
                "expected +, - or ]"
            );
        }

        panic!("unclosed memory operand");
    }

    /// Expects a `-` to already be consumed.
    fn parse_negative_number(&mut self) -> Operand {
        let mut positive = false;
        while let Some(Token::Minus) = self.peek() {
            self.next();
            positive = !positive;
        }

        let Token::Number(num) = *self.consume() else {
            panic!("expected number");
        };

        Operand::Imm(if positive { num } else { -num })
    }

    fn parse_operand(&mut self) -> Operand {
        let tok = self.consume().clone();
        match tok {
            Token::Reg(reg) => Operand::Reg(reg),
            Token::Byte | Token::Word | Token::Dword | Token::Qword => {
                Operand::Mem(self.parse_memory(Size::try_from(tok).unwrap()))
            }
            Token::Number(num) => Operand::Imm(num),
            Token::Minus => self.parse_negative_number(),
            _ => panic!("unexpected token"),
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

    fn get_instr_idx(&mut self) -> usize {
        let label = match self.next().expect("expected label name") {
            Token::Opcode(op) => op.as_str(),
            Token::Label(Label(name)) => name,
            Token::Sublabel(Label(name)) => &(self.parent_label.0.clone() + name),
            _ => panic!("not a valid label name"),
        };

        let Some(instr_idx) = self.labels.get(&Label(label.to_string())).copied() else {
            panic!("no label {label} found");
        };

        instr_idx
    }

    fn parse_mov(&mut self) {
        let dest = RM::try_from(self.parse_operand()).expect("expected register or memory operand");
        assert_eq!(self.next(), Some(&Token::Comma), "expected comma");

        let src = self.parse_operand();
        match src {
            Operand::Reg(reg) => {
                assert!(dest.size() == reg.size(), "operands are not the same size");
            }
            Operand::Mem(mem) => {
                assert!(dest.size() == mem.size, "operands are not the same size");
            }
            Operand::Imm(imm) => {
                assert!(
                    dest.size().bits() >= imm.bits(),
                    "source does not fit into destination"
                );
            }
        }

        match dest {
            RM::Reg(reg) => self.instrs.push(Instr::Mov {
                dest: reg,
                src: RMI64::try_from(src).expect("value out of range for qword"),
            }),
            RM::Mem(mem) => self.instrs.push(Instr::MovMem {
                dest: mem,
                src: match RI32::try_from(src) {
                    Ok(source) => source,
                    Err(RIConversionError::NotRegOrImm) => {
                        panic!("expected register or immediate")
                    }
                    Err(RIConversionError::ValueOutOfRange) => {
                        panic!("value out of range for dword")
                    }
                },
            }),
        }
    }

    fn parse_binary_op(&mut self, op: Opcode) {
        type O = Opcode;
        let dest = RM::try_from(self.parse_operand()).expect("expected register or memory operand");
        assert_eq!(self.next(), Some(&Token::Comma), "expected comma");

        let src = self.parse_operand();
        match src {
            Operand::Reg(reg) => {
                assert!(dest.size() == reg.size(), "operands are not the same size");
            }
            Operand::Mem(mem) => {
                assert!(dest.size() == mem.size, "operands are not the same size");
            }
            Operand::Imm(imm) => {
                assert!(
                    dest.size().bits() >= imm.bits(),
                    "source does not fit into destination"
                );
            }
        }
        let instr = match dest {
            RM::Reg(reg) => match op {
                O::Add => Instr::Add {
                    dest: reg,
                    src: RMI32::try_from(src).expect("value out of range for dword"),
                },
                O::Sub => Instr::Sub {
                    dest: reg,
                    src: RMI32::try_from(src).expect("value out of range for dword"),
                },
                O::Xor => Instr::Xor {
                    dest: reg,
                    src: RMI32::try_from(src).expect("value out of range for dword"),
                },
                O::Cmp => Instr::Cmp {
                    dest: reg,
                    src: RMI32::try_from(src).expect("value out of range for dword"),
                },
                _ => unreachable!("you forgot to add a case in parse_opcode"),
            },
            RM::Mem(mem) => match op {
                O::Add => Instr::AddMem {
                    dest: mem,
                    src: RI32::try_from(src).unwrap(),
                },
                O::Sub => Instr::SubMem {
                    dest: mem,
                    src: RI32::try_from(src).unwrap(),
                },
                O::Xor => Instr::XorMem {
                    dest: mem,
                    src: RI32::try_from(src).unwrap(),
                },
                O::Cmp => Instr::CmpMem {
                    dest: mem,
                    src: RI32::try_from(src).unwrap(),
                },
                _ => unreachable!("you forgot to add a case in parse_opcode"),
            },
        };

        self.instrs.push(instr);
    }

    fn next(&mut self) -> Option<&'a Token> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&'a Token> {
        self.tokens.peek().copied()
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
    use crate::operands::{
        DwordIndexReg, DwordReg, Imm32, Imm64, IndexReg, QwordIndexReg, QwordReg, RMI64, Reg,
        Scale, Size,
    };

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
                dest: Reg::Qword(QwordReg::Rax),
                src: RMI64::Reg(Reg::Qword(QwordReg::Rbx))
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
                dest: Reg::Qword(QwordReg::Rax),
                src: RMI32::Imm(Imm32(8))
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
                    dest: Reg::Qword(QwordReg::Rax),
                    src: RMI32::Imm(Imm32(8))
                },
                Instr::Xor {
                    dest: Reg::Qword(QwordReg::Rax),
                    src: RMI32::Reg(Reg::Qword(QwordReg::Rax)),
                },
                Instr::Sub {
                    dest: Reg::Qword(QwordReg::Rbx),
                    src: RMI32::Reg(Reg::Qword(QwordReg::Rax)),
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

    #[test]
    fn negative_number() {
        let source = "
    mov rax, -100
";

        let parsed = parse(source);
        assert_eq!(
            parsed.instrs,
            vec![Instr::Mov {
                dest: Reg::Qword(QwordReg::Rax),
                src: RMI64::Imm(Imm64((-100_i64).cast_unsigned()))
            }]
        );
    }

    #[test]
    #[should_panic(expected = "unexpected token")]
    fn double_colon_label() {
        let source = "
    label::
";
        let _ = parse(source);
    }

    #[test]
    fn memory_rsp() {
        let source = "
    mov qword [rsp], rax
";
        let parsed = parse(source);
        assert_eq!(
            parsed.instrs,
            vec![Instr::MovMem {
                dest: Mem {
                    base: Some(BaseReg::Qword(QwordReg::Rsp)),
                    index: None,
                    disp: None,
                    size: Size::Qword
                },
                src: RI32::Reg(Reg::Qword(QwordReg::Rax)),
            }]
        );
    }

    #[test]
    fn memory_index() {
        let source = "
    mov dword [rax*8], ebx
";
        let parsed = parse(source);
        assert_eq!(
            parsed.instrs,
            vec![Instr::MovMem {
                dest: Mem {
                    base: None,
                    index: Some(Index {
                        index: IndexReg::Qword(QwordIndexReg::Rax),
                        scale: Scale::Eight
                    }),
                    disp: None,
                    size: Size::Dword
                },
                src: RI32::Reg(Reg::Dword(DwordReg::Ebx)),
            }]
        );
    }

    #[test]
    #[should_panic(expected = "source does not fit into destination")]
    fn size_not_equal() {
        let source = "
    mov byte [eax], 256
";
        let _ = parse(source);
    }

    #[test]
    #[should_panic(expected = "source does not fit into destination")]
    fn negative_does_not_fit() {
        let source = "
    mov al, -129
";
        let _ = parse(source);
    }
}
