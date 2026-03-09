use phf::phf_map;

use crate::operands::{Label, Reg};
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

        while !self.is_at_end() {
            match self.current() {
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
                    tokens.push(Token::Colon);
                    self.pos += 1;
                }
                '.' => {
                    let mut scanned = String::from(".");
                    self.pos += 1;
                    assert!(self.current() != '.', "invalid sublabel name");

                    while !self.is_at_end() && self.current().is_alphanumeric() {
                        scanned.push(self.current());
                        self.pos += 1;
                    }

                    tokens.push(Token::Sublabel(Label(scanned)));
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.pos += 1;
                }
                '0'..='9' | '-' => {
                    let mut positive = true;
                    while !self.is_at_end() && self.current() == '-' {
                        positive = !positive;
                        self.pos += 1;
                    }

                    let mut scanned = String::new();
                    while !self.is_at_end() && self.current().is_numeric() {
                        scanned.push(self.current());
                        self.pos += 1;
                    }

                    let number = scanned.parse().expect("number out of range");
                    tokens.push(Token::Imm(if positive { number } else { -number }));
                }
                'a'..='z' => {
                    let mut scanned = String::new();
                    while !self.is_at_end() && self.current().is_alphanumeric() {
                        scanned.push(self.current());
                        self.pos += 1;
                    }

                    if let Some(token) = TOKENLOOKUP.get(&scanned).cloned() {
                        tokens.push(token);
                    } else {
                        tokens.push(Token::Label(Label(scanned)));
                    }
                }
                _ => panic!("unknown character: {}", self.current()),
            }
        }

        tokens
    }

    fn is_at_end(&self) -> bool {
        self.pos == self.chars.len()
    }

    fn current(&self) -> char {
        self.chars[self.pos]
    }
}

pub static TOKENLOOKUP: phf::Map<&str, Token> = phf_map! {
    "mov" => Token::Opcode(Opcode::Mov),
    "add" => Token::Opcode(Opcode::Add),
    "sub" => Token::Opcode(Opcode::Sub),
    "xor" => Token::Opcode(Opcode::Xor),
    "cmp" => Token::Opcode(Opcode::Cmp),
    "jmp" => Token::Opcode(Opcode::Jmp),
    "je" => Token::Opcode(Opcode::Je),
    "jz" => Token::Opcode(Opcode::Jz),
    "jne" => Token::Opcode(Opcode::Jne),
    "jnz" => Token::Opcode(Opcode::Jnz),
    "ja" => Token::Opcode(Opcode::Ja),
    "jnbe" => Token::Opcode(Opcode::Jnbe),
    "jae" => Token::Opcode(Opcode::Jae),
    "jnb" => Token::Opcode(Opcode::Jnb),
    "jb" => Token::Opcode(Opcode::Jb),
    "jnae" => Token::Opcode(Opcode::Jnae),
    "jbe" => Token::Opcode(Opcode::Jbe),
    "jna" => Token::Opcode(Opcode::Jna),
    "jg" => Token::Opcode(Opcode::Jg),
    "jnle" => Token::Opcode(Opcode::Jnle),
    "jge" => Token::Opcode(Opcode::Jge),
    "jnl" => Token::Opcode(Opcode::Jnl),
    "jl" => Token::Opcode(Opcode::Jl),
    "jnge" => Token::Opcode(Opcode::Jnge),
    "jle" => Token::Opcode(Opcode::Jle),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operands::Label;

    fn lex(src: &str) -> Vec<Token> {
        let lexer = Lexer::new(src);
        lexer.lex()
    }

    #[test]
    fn number() {
        let src = "999";
        let out = lex(src);
        assert_eq!(out[0], Token::Imm(999));
    }

    #[test]
    #[should_panic(expected = "number out of range")]
    fn number_out_of_range() {
        let src = "9999999999999999999999999999999999999999999999";
        lex(src);
    }

    #[test]
    fn valid_tokens() {
        let src = "mov add sub xor rax rbx";
        let out = lex(src);
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
    fn valid_instr() {
        let src = "mov rax, rbx";
        let out = lex(src);
        assert_eq!(
            out,
            vec![
                Token::Opcode(Opcode::Mov),
                Token::Reg(Reg::Rax),
                Token::Comma,
                Token::Reg(Reg::Rbx)
            ]
        );
    }

    #[test]
    fn valid_instrs() {
        let src = "
    mov rax, rbx 
    xor rbx, rax
";
        let out = lex(src);
        assert_eq!(
            out,
            vec![
                Token::Opcode(Opcode::Mov),
                Token::Reg(Reg::Rax),
                Token::Comma,
                Token::Reg(Reg::Rbx),
                Token::Opcode(Opcode::Xor),
                Token::Reg(Reg::Rbx),
                Token::Comma,
                Token::Reg(Reg::Rax)
            ]
        );
    }

    #[test]
    fn label() {
        let src = "
        label:
        xor rax, rax
        .if:
        add rax, 8
        .:
        jmp .
    ";

        let out = lex(src);
        assert_eq!(
            out,
            vec![
                Token::Label(Label("label".into())),
                Token::Colon,
                Token::Opcode(Opcode::Xor),
                Token::Reg(Reg::Rax),
                Token::Comma,
                Token::Reg(Reg::Rax),
                Token::Sublabel(Label(".if".into())),
                Token::Colon,
                Token::Opcode(Opcode::Add),
                Token::Reg(Reg::Rax),
                Token::Comma,
                Token::Imm(8),
                Token::Sublabel(Label(".".into())),
                Token::Colon,
                Token::Opcode(Opcode::Jmp),
                Token::Sublabel(Label(".".into()))
            ]
        );
    }

    #[test]
    fn label_with_number() {
        let src = "
    label1:
";
        let out = lex(src);
        assert_eq!(
            out,
            vec![Token::Label(Label("label1".into())), Token::Colon]
        );
    }

    #[test]
    fn jmp() {
        let src = "
    jmp label
    jmp .label
    ";

        let out = lex(src);
        assert_eq!(
            out,
            vec![
                Token::Opcode(Opcode::Jmp),
                Token::Label(Label("label".into())),
                Token::Opcode(Opcode::Jmp),
                Token::Sublabel(Label(".label".into()))
            ]
        );
    }

    #[test]
    fn negative_number() {
        let src = "-100 --100 ---100
";

        let out = lex(src);
        assert_eq!(
            out,
            vec![Token::Imm(-100), Token::Imm(100), Token::Imm(-100)]
        );
    }

    #[test]
    #[should_panic(expected = "number out of range")]
    fn naked_negations() {
        let src = "- --100 ---100
";

        let _ = lex(src);
    }

    #[test]
    #[should_panic(expected = "invalid sublabel name")]
    fn invalid_sublabel() {
        let src = "..label:";

        let _ = lex(src);
    }
}
