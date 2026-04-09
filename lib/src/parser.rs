use std::{collections::HashMap, iter::Peekable, slice::Iter};

use crate::{
    instruction::Instr,
    operands::{
        BaseReg, Bits, Imm32, Index, IndexReg, Label, Mem, Operand, OperandParseResult,
        OperandWithImmError, RI32, RM, RMI32, RMI64, Scale, Size,
    },
    syntax_error::{SyntaxError, SyntaxErrorKind},
    tokens::{Opcode, Token, TokenType},
};

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    labels: HashMap<Label, usize>,
    errors: Vec<SyntaxError>,
    parent_label: Label,
    line: usize,
}

// On error, we skip to the next newline and start parsing from there. If we reach the end, we
// stop parsing.
// The utility functions (`parse_operand`, `get_instr_idx` etc.) don't skip to the end of the line,
// so you have more flexibility handling the error.
impl<'a> Parser<'a> {
    pub fn new(tokens: Iter<'a, Token>, labels: HashMap<Label, usize>) -> Self {
        Self {
            tokens: tokens.peekable(),
            labels,
            errors: Vec::new(),
            parent_label: Label(String::new()),
            line: 1,
        }
    }

    pub fn parse(mut self) -> Result<Vec<Instr>, Vec<SyntaxError>> {
        let mut instrs = Vec::new();
        while let Some(token) = self.next() {
            match token.ty {
                TokenType::Opcode(op) => {
                    match self.parse_opcode(op) {
                        Some(instr) if self.peek().is_none_or(|t| t.ty == TokenType::Newline) => {
                            instrs.push(instr);
                        }
                        // If no errors occured during parsing, but there is an unexpected token
                        // after it.
                        Some(_) => {
                            self.errors.push(SyntaxError {
                                line: token.line,
                                error: SyntaxErrorKind::ExpectedNewline,
                            });
                            self.skip_to_newline_or_end();
                        }
                        None => {
                            self.skip_to_newline_or_end();
                        }
                    }
                }
                TokenType::Label(ref label) => {
                    self.parent_label.clone_from(label);
                    // Consume the ":".
                    self.next();
                }
                TokenType::Sublabel(_) => {
                    // Consume the ":".
                    self.next();
                }
                TokenType::Newline => {}
                _ => {
                    self.errors.push(SyntaxError {
                        line: token.line,
                        error: SyntaxErrorKind::UnexpectedToken,
                    });
                    self.skip_to_newline_or_end();
                }
            }
        }

        if self.errors.is_empty() {
            Ok(instrs)
        } else {
            Err(self.errors)
        }
    }

    fn skip_to_newline_or_end(&mut self) {
        while self.next().is_some_and(|t| t.ty != TokenType::Newline) {}
    }

    fn parse_opcode(&mut self, op: Opcode) -> Option<Instr> {
        type O = Opcode;
        let instr = match op {
            O::Mov => self.parse_mov()?,
            O::Add | O::Sub | O::Xor | O::Cmp => self.parse_binary_op(op)?,
            O::Jmp => {
                let dest = self.get_instr_idx()?;

                Instr::Jmp { dest }
            }
            O::Je | O::Jz => {
                let dest = self.get_instr_idx()?;

                Instr::Je { dest }
            }
            O::Jne | O::Jnz => {
                let dest = self.get_instr_idx()?;

                Instr::Jne { dest }
            }
            O::Ja | O::Jnbe => {
                let dest = self.get_instr_idx()?;

                Instr::Ja { dest }
            }
            O::Jae | O::Jnb => {
                let dest = self.get_instr_idx()?;

                Instr::Jae { dest }
            }
            O::Jb | O::Jnae => {
                let dest = self.get_instr_idx()?;

                Instr::Jb { dest }
            }
            O::Jbe | O::Jna => {
                let dest = self.get_instr_idx()?;

                Instr::Jbe { dest }
            }
            O::Jg | O::Jnle => {
                let dest = self.get_instr_idx()?;

                Instr::Jg { dest }
            }
            O::Jge | O::Jnl => {
                let dest = self.get_instr_idx()?;

                Instr::Jge { dest }
            }
            O::Jl | O::Jnge => {
                let dest = self.get_instr_idx()?;

                Instr::Jl { dest }
            }
            O::Jle | O::Jng => {
                let dest = self.get_instr_idx()?;

                Instr::Jle { dest }
            }
        };

        Some(instr)
    }

    // TODO: I KNOW CLIPPY (refactor)
    #[allow(clippy::too_many_lines)]
    fn parse_memory(&mut self, size: Size) -> Option<Mem> {
        if self.next().is_none_or(|t| t.ty != TokenType::LBracket) {
            return None;
        }

        let mut base = None;
        let mut index = None;
        let mut disp = None;

        let mut positive = true;
        let mut disp_overflow = false;
        while let Some(tok) = self.next() {
            match tok.ty {
                TokenType::RBracket => {
                    if base.is_none() && index.is_none() && disp.is_none() {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::IncompleteMemoryOperand,
                        });
                        return None;
                    } else if disp_overflow {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::DisplacementExceedsSignedDwordBounds,
                        });
                        return None;
                    }

                    return Some(Mem {
                        base,
                        index,
                        disp,
                        size,
                    });
                }
                // If we encounter a - or a +, then we start over, as we would fail the check at
                // the end if it was something like + rax (we would consume the + and panic because
                // the next token is not a +, a - or a ]).
                TokenType::Minus => {
                    positive = !positive;
                    continue;
                }
                TokenType::Plus => {
                    continue;
                }
                TokenType::Reg(reg) if self.peek().is_none_or(|t| t.ty != TokenType::Star) => {
                    if base.is_none() {
                        if let Ok(base_reg) = BaseReg::try_from(reg) {
                            base = Some(base_reg);
                        } else {
                            self.errors.push(SyntaxError {
                                line: self.line,
                                error: SyntaxErrorKind::InvalidBaseRegister,
                            });
                        }
                    } else {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::BaseRegisterAlreadyExists,
                        });
                    }
                }
                // We can call .next() here, since we already covered the case in which we don't
                // have a star.
                TokenType::Reg(reg) if self.next().is_some_and(|t| t.ty == TokenType::Star) => {
                    if index.is_some() {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::IndexRegisterAlreadyExists,
                        });
                    }

                    if let Some(scale_factor_token) = self.next()
                        && let TokenType::Number(imm) = scale_factor_token.ty
                    {
                        let Ok(index_reg) = IndexReg::try_from(reg) else {
                            self.errors.push(SyntaxError {
                                line: self.line,
                                error: SyntaxErrorKind::RspCannotBeUsedAsIndexRegister,
                            });
                            continue;
                        };

                        let Ok(scale) = Scale::try_from(imm) else {
                            self.errors.push(SyntaxError {
                                line: self.line,
                                error: SyntaxErrorKind::InvalidScaleFactor,
                            });
                            continue;
                        };

                        index = Some(Index {
                            index: index_reg,
                            scale,
                        });
                    } else {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::ExpectedScaleFactor,
                        });
                        return None;
                    }
                }
                TokenType::Number(val) => {
                    let Ok(disp_value) = i32::try_from(val) else {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::DisplacementOutOfRange,
                        });
                        continue;
                    };

                    let (new_disp, overflow) = disp
                        .unwrap_or(Imm32(0))
                        .0
                        .cast_signed()
                        .overflowing_add(disp_value);

                    disp_overflow = overflow;
                    disp = Some(Imm32(new_disp.cast_unsigned()));
                }
                _ => {
                    self.errors.push(SyntaxError {
                        line: self.line,
                        error: SyntaxErrorKind::UnexpectedToken,
                    });
                    return None;
                }
            }

            if self.peek().is_none_or(|t| {
                !matches!(
                    t.ty,
                    TokenType::Plus | TokenType::Minus | TokenType::RBracket
                )
            }) {
                self.errors.push(SyntaxError {
                    line: self.line,
                    error: SyntaxErrorKind::ExpectedPlusOrMinusOrRBracket,
                });
                return None;
            }
        }

        self.errors.push(SyntaxError {
            line: self.line,
            error: SyntaxErrorKind::UnclosedMemoryOperand,
        });
        None
    }

    /// Expects a `-` to already be consumed.
    fn parse_negative_number(&mut self) -> Option<Operand> {
        let mut positive = false;
        while self.peek().is_some_and(|t| t.ty == TokenType::Minus) {
            self.next();
            positive = !positive;
        }

        if let TokenType::Number(num) = self.next()?.ty {
            Some(Operand::Imm(if positive { num } else { -num }))
        } else {
            None
        }
    }

    fn parse_operand(&mut self) -> OperandParseResult {
        let Some(tok) = self.next() else {
            return OperandParseResult::Ok(None);
        };

        match tok.ty {
            TokenType::Reg(reg) => OperandParseResult::Ok(Some(Operand::Reg(reg))),
            TokenType::Byte | TokenType::Word | TokenType::Dword | TokenType::Qword => {
                OperandParseResult::Ok(Some(Operand::Mem(
                    match self.parse_memory(Size::try_from(tok.ty.clone()).unwrap()) {
                        Some(mem) => mem,
                        None => return OperandParseResult::ParsingError,
                    },
                )))
            }
            TokenType::Number(num) => OperandParseResult::Ok(Some(Operand::Imm(num))),
            TokenType::Minus => OperandParseResult::Ok(match self.parse_negative_number() {
                Some(num) => Some(num),
                None => return OperandParseResult::ParsingError,
            }),
            _ => OperandParseResult::Ok(None),
        }
    }

    fn get_instr_idx(&mut self) -> Option<usize> {
        let Some(token) = self.next() else {
            self.errors.push(SyntaxError {
                line: self.line,
                error: SyntaxErrorKind::ExpectedLabel,
            });

            return None;
        };

        let label = match &token.ty {
            TokenType::Opcode(op) => Label(op.as_str().to_string()),
            TokenType::Label(label) => label.clone(),
            TokenType::Sublabel(Label(name)) => Label(self.parent_label.0.clone() + name),
            _ => {
                self.errors.push(SyntaxError {
                    line: self.line,
                    error: SyntaxErrorKind::InvalidLabelName,
                });

                return None;
            }
        };

        let Some(instr_idx) = self.labels.get(&label).copied() else {
            self.errors.push(SyntaxError {
                line: self.line,
                error: SyntaxErrorKind::NoSuchLabel,
            });

            return None;
        };

        Some(instr_idx)
    }

    fn parse_mov(&mut self) -> Option<Instr> {
        let Ok(dest) = RM::try_from(self.parse_operand()?) else {
            self.errors.push(SyntaxError {
                line: self.line,
                error: SyntaxErrorKind::ExpectedRegisterOrMemory,
            });
            return None;
        };

        if self.next().is_none_or(|t| t.ty != TokenType::Comma) {
            self.errors.push(SyntaxError {
                line: self.line,
                error: SyntaxErrorKind::ExpectedComma,
            });
        }

        let src = self.parse_operand()?;
        match src {
            Some(Operand::Reg(reg)) if dest.size() != reg.size() => {
                self.errors.push(SyntaxError {
                    line: self.line,
                    error: SyntaxErrorKind::OperandSizeMismatch,
                });
            }
            Some(Operand::Mem(mem)) if dest.size() != mem.size => {
                self.errors.push(SyntaxError {
                    line: self.line,
                    error: SyntaxErrorKind::OperandSizeMismatch,
                });
            }
            Some(Operand::Imm(imm)) if dest.size().bits() < imm.bits() => {
                self.errors.push(SyntaxError {
                    line: self.line,
                    error: SyntaxErrorKind::SourceDoesNotFitIntoDestination,
                });
            }
            // If the operand is wrong, we don't care if it fits, we let the conversions fail and
            // return there.
            _ => {}
        }

        let instr = match dest {
            RM::Reg(reg) => Instr::Mov {
                dest: reg,
                src: match RMI64::try_from(src) {
                    Ok(source) => source,
                    Err(OperandWithImmError::WrongOperand) => {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::ExpectedRegisterMemoryOrImmediate,
                        });
                        return None;
                    }
                    Err(OperandWithImmError::ImmediateOutOfRange) => {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::ValueOutOfRangeForQword,
                        });
                        return None;
                    }
                },
            },
            RM::Mem(mem) => Instr::MovMem {
                dest: mem,
                src: match RI32::try_from(src) {
                    Ok(source) => source,
                    Err(OperandWithImmError::WrongOperand) => {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::ExpectedRegisterOrImmediate,
                        });
                        return None;
                    }
                    Err(OperandWithImmError::ImmediateOutOfRange) => {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::ValueOutOfRangeForDword,
                        });
                        return None;
                    }
                },
            },
        };

        Some(instr)
    }

    fn parse_binary_op(&mut self, op: Opcode) -> Option<Instr> {
        type O = Opcode;
        let Ok(dest) = RM::try_from(self.parse_operand()?) else {
            self.errors.push(SyntaxError {
                line: self.line,
                error: SyntaxErrorKind::ExpectedRegisterOrMemory,
            });
            return None;
        };
        if self.next().is_none_or(|t| t.ty != TokenType::Comma) {
            self.errors.push(SyntaxError {
                line: self.line,
                error: SyntaxErrorKind::ExpectedComma,
            });
        }

        let src = self.parse_operand()?;
        match src {
            Some(Operand::Reg(reg)) if dest.size() != reg.size() => {
                self.errors.push(SyntaxError {
                    line: self.line,
                    error: SyntaxErrorKind::OperandSizeMismatch,
                });
            }
            Some(Operand::Mem(mem)) if dest.size() != mem.size => {
                self.errors.push(SyntaxError {
                    line: self.line,
                    error: SyntaxErrorKind::OperandSizeMismatch,
                });
            }
            Some(Operand::Imm(imm)) if dest.size().bits() < imm.bits() => {
                self.errors.push(SyntaxError {
                    line: self.line,
                    error: SyntaxErrorKind::SourceDoesNotFitIntoDestination,
                });
            }
            // If the operand is wrong, we don't care if it fits, we let the conversions fail and
            // return there.
            _ => {}
        }

        // TODO: refactor
        let instr = match dest {
            RM::Reg(reg) => {
                let src = match RMI32::try_from(src) {
                    Ok(source) => source,
                    Err(OperandWithImmError::WrongOperand) => {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::ExpectedRegisterMemoryOrImmediate,
                        });
                        return None;
                    }
                    Err(OperandWithImmError::ImmediateOutOfRange) => {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::ValueOutOfRangeForDword,
                        });
                        return None;
                    }
                };
                match op {
                    O::Add => Instr::Add { dest: reg, src },
                    O::Sub => Instr::Sub { dest: reg, src },
                    O::Xor => Instr::Xor { dest: reg, src },
                    O::Cmp => Instr::Cmp { dest: reg, src },
                    _ => unreachable!("you forgot to add a case in parse_opcode"),
                }
            }
            RM::Mem(mem) => {
                let src = match RI32::try_from(src) {
                    Ok(source) => source,
                    Err(OperandWithImmError::WrongOperand) => {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::ExpectedRegisterOrImmediate,
                        });
                        return None;
                    }
                    Err(OperandWithImmError::ImmediateOutOfRange) => {
                        self.errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::ValueOutOfRangeForDword,
                        });
                        return None;
                    }
                };
                match op {
                    O::Add => Instr::AddMem { dest: mem, src },
                    O::Sub => Instr::SubMem { dest: mem, src },
                    O::Xor => Instr::XorMem { dest: mem, src },
                    O::Cmp => Instr::CmpMem { dest: mem, src },
                    _ => unreachable!("you forgot to add a case in parse_opcode"),
                }
            }
        };

        Some(instr)
    }

    fn next(&mut self) -> Option<&'a Token> {
        self.tokens.next().inspect(|t| {
            if t.ty == TokenType::Newline {
                self.line += 1;
            }
        })
    }

    fn peek(&mut self) -> Option<&'a Token> {
        self.tokens.peek().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instr;
    use crate::label_parser::{LabelParser, fix_opcode_label_definitions};
    use crate::lexer::Lexer;
    use crate::operands::{
        DwordReg, Imm32, Imm64, IndexReg, QwordIndexReg, QwordReg, RMI64, Reg, Scale, Size,
    };

    fn parse(source: &str) -> Result<Vec<Instr>, Vec<SyntaxError>> {
        let lexer = Lexer::new(source);
        let mut tokens = lexer.lex().unwrap();
        fix_opcode_label_definitions(&mut tokens);
        let labels = LabelParser::new(tokens.iter()).parse().unwrap();
        let parser = Parser::new(tokens.iter(), labels);

        parser.parse()
    }

    #[test]
    fn single_mov() {
        let source = "mov rax, rbx";
        let parsed = parse(source).unwrap();
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
        let parsed = parse(source).unwrap();
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
        let parsed = parse(source).unwrap();
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
    fn invalid_operand() {
        let source = "add 8, rax";
        let errors = parse(source).unwrap_err();

        assert_eq!(
            errors,
            vec![SyntaxError {
                line: 1,
                error: SyntaxErrorKind::ExpectedRegisterOrMemory
            }]
        );
    }

    #[test]
    fn jmp() {
        let source = "
    label:
        jmp label
";

        let parsed = parse(source).unwrap();
        assert_eq!(parsed, vec![Instr::Jmp { dest: 0 }]);
    }

    #[test]
    fn jmp_not_exists() {
        let source = "jmp label";

        let errors = parse(source).unwrap_err();

        assert_eq!(
            errors,
            vec![SyntaxError {
                line: 1,
                error: SyntaxErrorKind::NoSuchLabel
            }]
        );
    }

    #[test]
    fn jmp_invalid_operand() {
        let source = "jmp 2dfa";

        let errors = parse(source).unwrap_err();

        assert_eq!(
            errors,
            vec![SyntaxError {
                line: 1,
                error: SyntaxErrorKind::InvalidLabelName
            }]
        );
    }

    #[test]
    fn negative_number() {
        let source = "
    mov rax, -100
";

        let parsed = parse(source).unwrap();
        assert_eq!(
            parsed,
            vec![Instr::Mov {
                dest: Reg::Qword(QwordReg::Rax),
                src: RMI64::Imm(Imm64((-100_i64).cast_unsigned()))
            }]
        );
    }

    #[test]
    fn double_colon_label() {
        let source = "label::";
        let errors = parse(source).unwrap_err();

        assert_eq!(
            errors,
            vec![SyntaxError {
                line: 1,
                error: SyntaxErrorKind::UnexpectedToken
            }]
        );
    }

    #[test]
    fn memory_rsp() {
        let source = "
    mov qword [rsp], rax
";
        let parsed = parse(source).unwrap();
        assert_eq!(
            parsed,
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
        let parsed = parse(source).unwrap();
        assert_eq!(
            parsed,
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
    fn does_not_fit() {
        let source = "mov byte [eax], 256";
        let errors = parse(source).unwrap_err();

        assert_eq!(
            errors,
            vec![SyntaxError {
                line: 1,
                error: SyntaxErrorKind::SourceDoesNotFitIntoDestination
            }]
        );
    }

    #[test]
    fn negative_does_not_fit() {
        let source = "mov al, -129";
        let errors = parse(source).unwrap_err();

        assert_eq!(
            errors,
            vec![SyntaxError {
                line: 1,
                error: SyntaxErrorKind::SourceDoesNotFitIntoDestination
            }]
        );
    }

    #[test]
    fn no_newline_after_instruction() {
        let source = "mov rax, 8 xor rax, rax";
        let errors = parse(source).unwrap_err();

        assert_eq!(
            errors,
            vec![SyntaxError {
                line: 1,
                error: SyntaxErrorKind::ExpectedNewline
            }]
        );
    }
}
