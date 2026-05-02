use phf::phf_map;

use crate::operands::{ByteReg, DwordReg, Label, QwordReg, Reg, WordReg};
use crate::syntax_error::{SyntaxError, SyntaxErrorKind};
use crate::tokens::{Opcode, Token, TokenType};

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    // TODO: shorten this probably
    #[allow(clippy::too_many_lines)]
    pub fn lex(mut self) -> Result<Vec<Token>, Vec<SyntaxError>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            match self.current() {
                '\n' => {
                    tokens.push(Token {
                        line: self.line,
                        ty: TokenType::Newline,
                    });
                    self.line += 1;
                    self.pos += 1;
                }
                c if c.is_whitespace() => {
                    self.pos += 1;
                }
                ';' => {
                    while !self.is_at_end() && self.current() != '\n' {
                        self.pos += 1;
                    }
                    self.pos += 1;
                }
                ':' => {
                    tokens.push(Token {
                        line: self.line,
                        ty: TokenType::Colon,
                    });
                    self.pos += 1;
                }
                '[' => {
                    tokens.push(Token {
                        line: self.line,
                        ty: TokenType::LBracket,
                    });
                    self.pos += 1;
                }
                ']' => {
                    tokens.push(Token {
                        line: self.line,
                        ty: TokenType::RBracket,
                    });
                    self.pos += 1;
                }
                '+' => {
                    tokens.push(Token {
                        line: self.line,
                        ty: TokenType::Plus,
                    });
                    self.pos += 1;
                }
                '-' => {
                    tokens.push(Token {
                        line: self.line,
                        ty: TokenType::Minus,
                    });
                    self.pos += 1;
                }
                '*' => {
                    tokens.push(Token {
                        line: self.line,
                        ty: TokenType::Star,
                    });
                    self.pos += 1;
                }
                ',' => {
                    tokens.push(Token {
                        line: self.line,
                        ty: TokenType::Comma,
                    });
                    self.pos += 1;
                }
                '.' => {
                    let mut scanned = String::from(".");
                    self.pos += 1;

                    while !self.is_at_end()
                        && !self.current().is_whitespace()
                        && !matches!(self.current(), ':' | '[' | ']' | '+' | '-' | '*' | ',')
                    {
                        scanned.push(self.current());
                        self.pos += 1;
                    }

                    if scanned.starts_with("..") {
                        errors.push(SyntaxError {
                            line: self.line,
                            error: SyntaxErrorKind::InvalidSublabelName,
                        });
                    } else {
                        tokens.push(Token {
                            line: self.line,
                            ty: TokenType::Sublabel(Label(scanned)),
                        });
                    }
                }
                '0'..='9' => {
                    let mut scanned = String::new();
                    while !self.is_at_end() && self.current().is_numeric() {
                        scanned.push(self.current());
                        self.pos += 1;
                    }

                    match scanned.parse() {
                        Ok(number) => {
                            tokens.push(Token {
                                line: self.line,
                                ty: TokenType::Number(number),
                            });
                        }
                        Err(_) => {
                            errors.push(SyntaxError {
                                line: self.line,
                                error: SyntaxErrorKind::NumberOutOfRange,
                            });
                        }
                    }
                }
                _ => {
                    let mut scanned = String::new();
                    while !self.is_at_end()
                        && !self.current().is_whitespace()
                        && !matches!(self.current(), ':' | '[' | ']' | '+' | '-' | '*' | ',')
                    {
                        scanned.push(self.current());
                        self.pos += 1;
                    }

                    let ty = if let Some(token_type) =
                        TOKENLOOKUP.get(&scanned.to_lowercase()).cloned()
                    {
                        token_type
                    } else {
                        TokenType::Label(Label(scanned))
                    };

                    tokens.push(Token {
                        line: self.line,
                        ty,
                    });
                }
            }
        }

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos == self.chars.len()
    }

    fn current(&self) -> char {
        self.chars[self.pos]
    }
}

pub static TOKENLOOKUP: phf::Map<&str, TokenType> = phf_map! {
    "mov" => TokenType::Opcode(Opcode::Mov),
    "add" => TokenType::Opcode(Opcode::Add),
    "sub" => TokenType::Opcode(Opcode::Sub),
    "xor" => TokenType::Opcode(Opcode::Xor),
    "cmp" => TokenType::Opcode(Opcode::Cmp),
    "jmp" => TokenType::Opcode(Opcode::Jmp),
    "je" => TokenType::Opcode(Opcode::Je),
    "jz" => TokenType::Opcode(Opcode::Jz),
    "jne" => TokenType::Opcode(Opcode::Jne),
    "jnz" => TokenType::Opcode(Opcode::Jnz),
    "ja" => TokenType::Opcode(Opcode::Ja),
    "jnbe" => TokenType::Opcode(Opcode::Jnbe),
    "jae" => TokenType::Opcode(Opcode::Jae),
    "jnb" => TokenType::Opcode(Opcode::Jnb),
    "jb" => TokenType::Opcode(Opcode::Jb),
    "jnae" => TokenType::Opcode(Opcode::Jnae),
    "jbe" => TokenType::Opcode(Opcode::Jbe),
    "jna" => TokenType::Opcode(Opcode::Jna),
    "jg" => TokenType::Opcode(Opcode::Jg),
    "jnle" => TokenType::Opcode(Opcode::Jnle),
    "jge" => TokenType::Opcode(Opcode::Jge),
    "jnl" => TokenType::Opcode(Opcode::Jnl),
    "jl" => TokenType::Opcode(Opcode::Jl),
    "jnge" => TokenType::Opcode(Opcode::Jnge),
    "jle" => TokenType::Opcode(Opcode::Jle),
    "rax" => TokenType::Reg(Reg::Qword(QwordReg::Rax)),
    "eax" => TokenType::Reg(Reg::Dword(DwordReg::Eax)),
    "ax" => TokenType::Reg(Reg::Word(WordReg::Ax)),
    "ah" => TokenType::Reg(Reg::Byte(ByteReg::Ah)),
    "al" => TokenType::Reg(Reg::Byte(ByteReg::Al)),
    "rbx" => TokenType::Reg(Reg::Qword(QwordReg::Rbx)),
    "ebx" => TokenType::Reg(Reg::Dword(DwordReg::Ebx)),
    "bx" => TokenType::Reg(Reg::Word(WordReg::Bx)),
    "bh" => TokenType::Reg(Reg::Byte(ByteReg::Bh)),
    "bl" => TokenType::Reg(Reg::Byte(ByteReg::Bl)),
    "rcx" => TokenType::Reg(Reg::Qword(QwordReg::Rcx)),
    "ecx" => TokenType::Reg(Reg::Dword(DwordReg::Ecx)),
    "cx" => TokenType::Reg(Reg::Word(WordReg::Cx)),
    "ch" => TokenType::Reg(Reg::Byte(ByteReg::Ch)),
    "cl" => TokenType::Reg(Reg::Byte(ByteReg::Cl)),
    "rdx" => TokenType::Reg(Reg::Qword(QwordReg::Rdx)),
    "edx" => TokenType::Reg(Reg::Dword(DwordReg::Edx)),
    "dx" => TokenType::Reg(Reg::Word(WordReg::Dx)),
    "dh" => TokenType::Reg(Reg::Byte(ByteReg::Dh)),
    "dl" => TokenType::Reg(Reg::Byte(ByteReg::Dl)),
    "rsi" => TokenType::Reg(Reg::Qword(QwordReg::Rsi)),
    "esi" => TokenType::Reg(Reg::Dword(DwordReg::Esi)),
    "si" => TokenType::Reg(Reg::Word(WordReg::Si)),
    "sil" => TokenType::Reg(Reg::Byte(ByteReg::Sil)),
    "rdi" => TokenType::Reg(Reg::Qword(QwordReg::Rdi)),
    "edi" => TokenType::Reg(Reg::Dword(DwordReg::Edi)),
    "di" => TokenType::Reg(Reg::Word(WordReg::Di)),
    "dil" => TokenType::Reg(Reg::Byte(ByteReg::Dil)),
    "rsp" => TokenType::Reg(Reg::Qword(QwordReg::Rsp)),
    "esp" => TokenType::Reg(Reg::Dword(DwordReg::Esp)),
    "sp" => TokenType::Reg(Reg::Word(WordReg::Sp)),
    "spl" => TokenType::Reg(Reg::Byte(ByteReg::Spl)),
    "rbp" => TokenType::Reg(Reg::Qword(QwordReg::Rbp)),
    "ebp" => TokenType::Reg(Reg::Dword(DwordReg::Ebp)),
    "bp" => TokenType::Reg(Reg::Word(WordReg::Bp)),
    "bpl" => TokenType::Reg(Reg::Byte(ByteReg::Bpl)),
    "r8" => TokenType::Reg(Reg::Qword(QwordReg::R8)),
    "r8d" => TokenType::Reg(Reg::Dword(DwordReg::R8d)),
    "r8w" => TokenType::Reg(Reg::Word(WordReg::R8w)),
    "r8b" => TokenType::Reg(Reg::Byte(ByteReg::R8b)),
    "r9" => TokenType::Reg(Reg::Qword(QwordReg::R9)),
    "r9d" => TokenType::Reg(Reg::Dword(DwordReg::R9d)),
    "r9w" => TokenType::Reg(Reg::Word(WordReg::R9w)),
    "r9b" => TokenType::Reg(Reg::Byte(ByteReg::R9b)),
    "r10" => TokenType::Reg(Reg::Qword(QwordReg::R10)),
    "r10d" => TokenType::Reg(Reg::Dword(DwordReg::R10d)),
    "r10w" => TokenType::Reg(Reg::Word(WordReg::R10w)),
    "r10b" => TokenType::Reg(Reg::Byte(ByteReg::R10b)),
    "r11" => TokenType::Reg(Reg::Qword(QwordReg::R11)),
    "r11d" => TokenType::Reg(Reg::Dword(DwordReg::R11d)),
    "r11w" => TokenType::Reg(Reg::Word(WordReg::R11w)),
    "r11b" => TokenType::Reg(Reg::Byte(ByteReg::R11b)),
    "r12" => TokenType::Reg(Reg::Qword(QwordReg::R12)),
    "r12d" => TokenType::Reg(Reg::Dword(DwordReg::R12d)),
    "r12w" => TokenType::Reg(Reg::Word(WordReg::R12w)),
    "r12b" => TokenType::Reg(Reg::Byte(ByteReg::R12b)),
    "r13" => TokenType::Reg(Reg::Qword(QwordReg::R13)),
    "r13d" => TokenType::Reg(Reg::Dword(DwordReg::R13d)),
    "r13w" => TokenType::Reg(Reg::Word(WordReg::R13w)),
    "r13b" => TokenType::Reg(Reg::Byte(ByteReg::R13b)),
    "r14" => TokenType::Reg(Reg::Qword(QwordReg::R14)),
    "r14d" => TokenType::Reg(Reg::Dword(DwordReg::R14d)),
    "r14w" => TokenType::Reg(Reg::Word(WordReg::R14w)),
    "r14b" => TokenType::Reg(Reg::Byte(ByteReg::R14b)),
    "r15" => TokenType::Reg(Reg::Qword(QwordReg::R15)),
    "r15d" => TokenType::Reg(Reg::Dword(DwordReg::R15d)),
    "r15w" => TokenType::Reg(Reg::Word(WordReg::R15w)),
    "r15b" => TokenType::Reg(Reg::Byte(ByteReg::R15b)),
    "byte" => TokenType::Byte,
    "word" => TokenType::Word,
    "dword" => TokenType::Dword,
    "qword" => TokenType::Qword,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operands::{Label, QwordReg};

    fn lex(src: &str) -> Result<Vec<Token>, Vec<SyntaxError>> {
        let lexer = Lexer::new(src);
        lexer.lex()
    }

    #[test]
    fn number() {
        let src = "999";
        let out = lex(src).unwrap();
        assert_eq!(
            out[0],
            Token {
                line: 1,
                ty: TokenType::Number(999)
            }
        );
    }

    #[test]
    fn number_out_of_range() {
        let src = "9999999999999999999999999999999999999999999999";
        let errors = lex(src).unwrap_err();
        assert_eq!(
            errors,
            vec![SyntaxError {
                line: 1,
                error: SyntaxErrorKind::NumberOutOfRange
            }]
        );
    }

    #[test]
    fn valid_tokens() {
        let src = "mov add sub xor rax rbx";
        let out = lex(src).unwrap();
        assert_eq!(
            out,
            vec![
                Token {
                    line: 1,
                    ty: TokenType::Opcode(Opcode::Mov)
                },
                Token {
                    line: 1,
                    ty: TokenType::Opcode(Opcode::Add)
                },
                Token {
                    line: 1,
                    ty: TokenType::Opcode(Opcode::Sub)
                },
                Token {
                    line: 1,
                    ty: TokenType::Opcode(Opcode::Xor)
                },
                Token {
                    line: 1,
                    ty: TokenType::Reg(Reg::Qword(QwordReg::Rax))
                },
                Token {
                    line: 1,
                    ty: TokenType::Reg(Reg::Qword(QwordReg::Rbx))
                },
            ]
        );
    }

    #[test]
    fn valid_instr() {
        let src = "mov rax, rbx";
        let out = lex(src).unwrap();
        assert_eq!(
            out,
            vec![
                Token {
                    line: 1,
                    ty: TokenType::Opcode(Opcode::Mov)
                },
                Token {
                    line: 1,
                    ty: TokenType::Reg(Reg::Qword(QwordReg::Rax))
                },
                Token {
                    line: 1,
                    ty: TokenType::Comma
                },
                Token {
                    line: 1,
                    ty: TokenType::Reg(Reg::Qword(QwordReg::Rbx))
                },
            ]
        );
    }

    #[test]
    fn valid_instrs() {
        let src = "mov rax, rbx
xor rbx, rax";
        let out = lex(src).unwrap();
        assert_eq!(
            out,
            vec![
                Token {
                    line: 1,
                    ty: TokenType::Opcode(Opcode::Mov)
                },
                Token {
                    line: 1,
                    ty: TokenType::Reg(Reg::Qword(QwordReg::Rax))
                },
                Token {
                    line: 1,
                    ty: TokenType::Comma
                },
                Token {
                    line: 1,
                    ty: TokenType::Reg(Reg::Qword(QwordReg::Rbx))
                },
                Token {
                    line: 1,
                    ty: TokenType::Newline
                },
                Token {
                    line: 2,
                    ty: TokenType::Opcode(Opcode::Xor)
                },
                Token {
                    line: 2,
                    ty: TokenType::Reg(Reg::Qword(QwordReg::Rbx))
                },
                Token {
                    line: 2,
                    ty: TokenType::Comma
                },
                Token {
                    line: 2,
                    ty: TokenType::Reg(Reg::Qword(QwordReg::Rax))
                },
            ]
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn label() {
        let src = "label:
xor rax, rax
.if:
add rax, 8
.:
jmp .";

        let out = lex(src).unwrap();
        assert_eq!(
            out,
            vec![
                Token {
                    line: 1,
                    ty: TokenType::Label(Label("label".into()))
                },
                Token {
                    line: 1,
                    ty: TokenType::Colon
                },
                Token {
                    line: 1,
                    ty: TokenType::Newline
                },
                Token {
                    line: 2,
                    ty: TokenType::Opcode(Opcode::Xor)
                },
                Token {
                    line: 2,
                    ty: TokenType::Reg(Reg::Qword(QwordReg::Rax))
                },
                Token {
                    line: 2,
                    ty: TokenType::Comma
                },
                Token {
                    line: 2,
                    ty: TokenType::Reg(Reg::Qword(QwordReg::Rax))
                },
                Token {
                    line: 2,
                    ty: TokenType::Newline
                },
                Token {
                    line: 3,
                    ty: TokenType::Sublabel(Label(".if".into()))
                },
                Token {
                    line: 3,
                    ty: TokenType::Colon
                },
                Token {
                    line: 3,
                    ty: TokenType::Newline
                },
                Token {
                    line: 4,
                    ty: TokenType::Opcode(Opcode::Add)
                },
                Token {
                    line: 4,
                    ty: TokenType::Reg(Reg::Qword(QwordReg::Rax))
                },
                Token {
                    line: 4,
                    ty: TokenType::Comma
                },
                Token {
                    line: 4,
                    ty: TokenType::Number(8)
                },
                Token {
                    line: 4,
                    ty: TokenType::Newline
                },
                Token {
                    line: 5,
                    ty: TokenType::Sublabel(Label(".".into()))
                },
                Token {
                    line: 5,
                    ty: TokenType::Colon
                },
                Token {
                    line: 5,
                    ty: TokenType::Newline
                },
                Token {
                    line: 6,
                    ty: TokenType::Opcode(Opcode::Jmp)
                },
                Token {
                    line: 6,
                    ty: TokenType::Sublabel(Label(".".into()))
                },
            ]
        );
    }

    #[test]
    fn single_dot_at_end() {
        let src = "jmp .";
        let out = lex(src).unwrap();
        assert_eq!(
            out,
            vec![
                Token {
                    line: 1,
                    ty: TokenType::Opcode(Opcode::Jmp)
                },
                Token {
                    line: 1,
                    ty: TokenType::Sublabel(Label(".".into()))
                },
            ]
        );
    }

    #[test]
    fn label_with_number() {
        let src = "label1:";
        let out = lex(src).unwrap();
        assert_eq!(
            out,
            vec![
                Token {
                    line: 1,
                    ty: TokenType::Label(Label("label1".into()))
                },
                Token {
                    line: 1,
                    ty: TokenType::Colon
                },
            ]
        );
    }

    #[test]
    fn jmp() {
        let src = "jmp label
jmp .label";

        let out = lex(src).unwrap();
        assert_eq!(
            out,
            vec![
                Token {
                    line: 1,
                    ty: TokenType::Opcode(Opcode::Jmp)
                },
                Token {
                    line: 1,
                    ty: TokenType::Label(Label("label".into()))
                },
                Token {
                    line: 1,
                    ty: TokenType::Newline
                },
                Token {
                    line: 2,
                    ty: TokenType::Opcode(Opcode::Jmp)
                },
                Token {
                    line: 2,
                    ty: TokenType::Sublabel(Label(".label".into()))
                },
            ]
        );
    }

    #[test]
    fn invalid_sublabel() {
        let src = "..label:";

        let errors = lex(src).unwrap_err();
        assert_eq!(
            errors[0],
            SyntaxError {
                line: 1,
                error: SyntaxErrorKind::InvalidSublabelName
            }
        );
    }

    #[test]
    fn number_with_letters() {
        let src = "3412fdsa4321";

        let tokens = lex(src).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    line: 1,
                    ty: TokenType::Number(3412)
                },
                Token {
                    line: 1,
                    ty: TokenType::Label(Label("fdsa4321".to_string()))
                },
            ]
        );
    }

    #[test]
    fn number_out_of_range_with_letters() {
        let src = "4213412351234134126325618345218184687653284f4321436218621845832165486321";

        let errors = lex(src).unwrap_err();
        assert_eq!(
            errors,
            vec![SyntaxError {
                line: 1,
                error: SyntaxErrorKind::NumberOutOfRange
            },]
        );
    }

    #[test]
    fn any_character() {
        let src = "under_score 猫";

        let tokens = lex(src).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    line: 1,
                    ty: TokenType::Label(Label("under_score".to_string()))
                },
                Token {
                    line: 1,
                    ty: TokenType::Label(Label("猫".to_string()))
                }
            ]
        );
    }
}
