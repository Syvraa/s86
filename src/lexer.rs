use phf::phf_map;

use crate::operands::Reg;
use crate::tokens::{Opcode, Token};

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
        }
    }

    pub fn lex(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        self.skip_whitespace();
        while !self.is_at_end() {
            match self.current() {
                ';' => {
                    while !self.is_at_end() && self.current() != '\n' {
                        self.pos += 1;
                    }
                    self.pos += 1;
                }
                '0'..='9' => {
                    let mut scanned = String::new();
                    while !self.is_at_end() && self.current().is_numeric() {
                        scanned.push(self.current());
                        self.pos += 1;
                    }

                    tokens.push(Token::Imm(scanned.parse().expect("number out of range")));
                }
                'a'..='z' => {
                    let mut scanned = String::new();
                    while !(self.is_at_end()
                        || self.current().is_whitespace()
                        || self.current() == ',')
                    {
                        scanned.push(self.current());
                        self.pos += 1;
                    }

                    tokens.push(
                        *TOKENLOOKUP
                            .get(&scanned)
                            .unwrap_or_else(|| panic!("not recognized: {}", &scanned)),
                    );
                }
                _ => panic!("unknown character: {}", self.current()),
            }
            // Skip after, otherwise if the last character is a newline, we would skip it, then try
            // to access out-of-bounds and crash.
            self.skip_whitespace();
        }

        tokens
    }

    fn is_at_end(&self) -> bool {
        self.pos == self.chars.len()
    }

    fn current(&self) -> char {
        self.chars[self.pos]
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && (self.current().is_whitespace() || self.current() == ',') {
            self.pos += 1;
        }
    }
}

static TOKENLOOKUP: phf::Map<&str, Token> = phf_map! {
    "mov" => Token::Opcode(Opcode::Mov),
    "add" => Token::Opcode(Opcode::Add),
    "sub" => Token::Opcode(Opcode::Sub),
    "xor" => Token::Opcode(Opcode::Xor),
    "rax" => Token::Reg(Reg::Rax),
    "rbx" => Token::Reg(Reg::Rbx),
    "rcx" => Token::Reg(Reg::Rcx),
    "rdx" => Token::Reg(Reg::Rdx),
    "rsi" => Token::Reg(Reg::Rsi),
    "rdi" => Token::Reg(Reg::Rdi),
    "rsp" => Token::Reg(Reg::Rsp),
    "rbp" => Token::Reg(Reg::Rbp),
    "r8" => Token::Reg(Reg::R8),
    "r9" => Token::Reg(Reg::R9),
    "r10" => Token::Reg(Reg::R10),
    "r11" => Token::Reg(Reg::R11),
    "r12" => Token::Reg(Reg::R12),
    "r13" => Token::Reg(Reg::R13),
    "r14" => Token::Reg(Reg::R14),
    "r15" => Token::Reg(Reg::R15),
};

mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let src = "999";
        let lexer = Lexer::new(src);
        let out = lexer.lex();
        let Token::Imm(num) = out[0] else {
            panic!("was not an immediate");
        };
        assert_eq!(num, 999);
    }

    #[test]
    #[should_panic(expected = "number out of range")]
    fn test_number_out_of_range() {
        let src = "9999999999999999999999999999999999999999999999";
        let lexer = Lexer::new(src);
        let out = lexer.lex();
        let Token::Imm(num) = out[0] else {
            panic!("was not an immediate");
        };
        assert_eq!(num, 999);
    }

    #[test]
    fn test_valid_tokens() {
        let src = "mov add sub xor rax rbx";
        let lexer = Lexer::new(src);
        let out = lexer.lex();
        assert_eq!(
            out,
            vec![
                Token::Opcode(Opcode::Mov),
                Token::Opcode(Opcode::Add),
                Token::Opcode(Opcode::Sub),
                Token::Opcode(Opcode::Xor),
                Token::Reg(Reg::Rax),
                Token::Reg(Reg::Rbx)
            ]
        );
    }

    #[test]
    #[should_panic(expected = "not recognized: dafhskgh")]
    fn test_invalid_token() {
        let src = "dafhskgh";
        let lexer = Lexer::new(src);
        _ = lexer.lex();
    }

    #[test]
    fn test_valid_instr() {
        let src = "mov rax, rbx";
        let lexer = Lexer::new(src);
        let out = lexer.lex();
        assert_eq!(
            out,
            vec![
                Token::Opcode(Opcode::Mov),
                Token::Reg(Reg::Rax),
                Token::Reg(Reg::Rbx)
            ]
        );
    }

    #[test]
    fn test_valid_instrs() {
        let src = "
    mov rax, rbx 
    xor rbx, rax
";
        let lexer = Lexer::new(src);
        let out = lexer.lex();
        assert_eq!(
            out,
            vec![
                Token::Opcode(Opcode::Mov),
                Token::Reg(Reg::Rax),
                Token::Reg(Reg::Rbx),
                Token::Opcode(Opcode::Xor),
                Token::Reg(Reg::Rbx),
                Token::Reg(Reg::Rax)
            ]
        );
    }
}
