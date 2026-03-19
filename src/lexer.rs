use phf::phf_map;

use crate::operands::{ByteReg, DwordReg, Label, QwordReg, Reg, WordReg};
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
                '[' => {
                    tokens.push(Token::LBracket);
                    self.pos += 1;
                }
                ']' => {
                    tokens.push(Token::RBracket);
                    self.pos += 1;
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.pos += 1;
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.pos += 1;
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.pos += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
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
                '0'..='9' => {
                    let mut scanned = String::new();
                    while !self.is_at_end() && self.current().is_numeric() {
                        scanned.push(self.current());
                        self.pos += 1;
                    }

                    let number = scanned.parse().expect("number out of range");
                    tokens.push(Token::Number(number));
                }
                'a'..='z' => {
                    let mut scanned = String::new();
                    while !self.is_at_end() && self.current().is_alphanumeric() {
                        scanned.push(self.current());
                        self.pos += 1;
                    }

                    if let Some(token) = TOKENLOOKUP.get(&scanned.to_lowercase()).cloned() {
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
    "rax" => Token::Reg(Reg::Qword(QwordReg::Rax)),
    "eax" => Token::Reg(Reg::Dword(DwordReg::Eax)),
    "ax" => Token::Reg(Reg::Word(WordReg::Ax)),
    "ah" => Token::Reg(Reg::Byte(ByteReg::Ah)),
    "al" => Token::Reg(Reg::Byte(ByteReg::Al)),
    "rbx" => Token::Reg(Reg::Qword(QwordReg::Rbx)),
    "ebx" => Token::Reg(Reg::Dword(DwordReg::Ebx)),
    "bx" => Token::Reg(Reg::Word(WordReg::Bx)),
    "bh" => Token::Reg(Reg::Byte(ByteReg::Bh)),
    "bl" => Token::Reg(Reg::Byte(ByteReg::Bl)),
    "rcx" => Token::Reg(Reg::Qword(QwordReg::Rcx)),
    "ecx" => Token::Reg(Reg::Dword(DwordReg::Ecx)),
    "cx" => Token::Reg(Reg::Word(WordReg::Cx)),
    "ch" => Token::Reg(Reg::Byte(ByteReg::Ch)),
    "cl" => Token::Reg(Reg::Byte(ByteReg::Cl)),
    "rdx" => Token::Reg(Reg::Qword(QwordReg::Rdx)),
    "edx" => Token::Reg(Reg::Dword(DwordReg::Edx)),
    "dx" => Token::Reg(Reg::Word(WordReg::Dx)),
    "dh" => Token::Reg(Reg::Byte(ByteReg::Dh)),
    "dl" => Token::Reg(Reg::Byte(ByteReg::Dl)),
    "rsi" => Token::Reg(Reg::Qword(QwordReg::Rsi)),
    "esi" => Token::Reg(Reg::Dword(DwordReg::Esi)),
    "si" => Token::Reg(Reg::Word(WordReg::Si)),
    "sil" => Token::Reg(Reg::Byte(ByteReg::Sil)),
    "rdi" => Token::Reg(Reg::Qword(QwordReg::Rdi)),
    "edi" => Token::Reg(Reg::Dword(DwordReg::Edi)),
    "di" => Token::Reg(Reg::Word(WordReg::Di)),
    "dil" => Token::Reg(Reg::Byte(ByteReg::Dil)),
    "rsp" => Token::Reg(Reg::Qword(QwordReg::Rsp)),
    "esp" => Token::Reg(Reg::Dword(DwordReg::Esp)),
    "sp" => Token::Reg(Reg::Word(WordReg::Sp)),
    "spl" => Token::Reg(Reg::Byte(ByteReg::Spl)),
    "rbp" => Token::Reg(Reg::Qword(QwordReg::Rbp)),
    "ebp" => Token::Reg(Reg::Dword(DwordReg::Ebp)),
    "bp" => Token::Reg(Reg::Word(WordReg::Bp)),
    "bpl" => Token::Reg(Reg::Byte(ByteReg::Bpl)),
    "r8" => Token::Reg(Reg::Qword(QwordReg::R8)),
    "r8d" => Token::Reg(Reg::Dword(DwordReg::R8d)),
    "r8w" => Token::Reg(Reg::Word(WordReg::R8w)),
    "r8b" => Token::Reg(Reg::Byte(ByteReg::R8b)),
    "r9" => Token::Reg(Reg::Qword(QwordReg::R9)),
    "r9d" => Token::Reg(Reg::Dword(DwordReg::R9d)),
    "r9w" => Token::Reg(Reg::Word(WordReg::R9w)),
    "r9b" => Token::Reg(Reg::Byte(ByteReg::R9b)),
    "r10" => Token::Reg(Reg::Qword(QwordReg::R10)),
    "r10d" => Token::Reg(Reg::Dword(DwordReg::R10d)),
    "r10w" => Token::Reg(Reg::Word(WordReg::R10w)),
    "r10b" => Token::Reg(Reg::Byte(ByteReg::R10b)),
    "r11" => Token::Reg(Reg::Qword(QwordReg::R11)),
    "r11d" => Token::Reg(Reg::Dword(DwordReg::R11d)),
    "r11w" => Token::Reg(Reg::Word(WordReg::R11w)),
    "r11b" => Token::Reg(Reg::Byte(ByteReg::R11b)),
    "r12" => Token::Reg(Reg::Qword(QwordReg::R12)),
    "r12d" => Token::Reg(Reg::Dword(DwordReg::R12d)),
    "r12w" => Token::Reg(Reg::Word(WordReg::R12w)),
    "r12b" => Token::Reg(Reg::Byte(ByteReg::R12b)),
    "r13" => Token::Reg(Reg::Qword(QwordReg::R13)),
    "r13d" => Token::Reg(Reg::Dword(DwordReg::R13d)),
    "r13w" => Token::Reg(Reg::Word(WordReg::R13w)),
    "r13b" => Token::Reg(Reg::Byte(ByteReg::R13b)),
    "r14" => Token::Reg(Reg::Qword(QwordReg::R14)),
    "r14d" => Token::Reg(Reg::Dword(DwordReg::R14d)),
    "r14w" => Token::Reg(Reg::Word(WordReg::R14w)),
    "r14b" => Token::Reg(Reg::Byte(ByteReg::R14b)),
    "r15" => Token::Reg(Reg::Qword(QwordReg::R15)),
    "r15d" => Token::Reg(Reg::Dword(DwordReg::R15d)),
    "r15w" => Token::Reg(Reg::Word(WordReg::R15w)),
    "r15b" => Token::Reg(Reg::Byte(ByteReg::R15b)),
    "byte" => Token::Byte,
    "word" => Token::Word,
    "dword" => Token::Dword,
    "qword" => Token::Qword,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operands::{Label, QwordReg};

    fn lex(src: &str) -> Vec<Token> {
        let lexer = Lexer::new(src);
        lexer.lex()
    }

    #[test]
    fn number() {
        let src = "999";
        let out = lex(src);
        assert_eq!(out[0], Token::Number(999));
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
                Token::Reg(Reg::Qword(QwordReg::Rax)),
                Token::Reg(Reg::Qword(QwordReg::Rbx))
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
                Token::Reg(Reg::Qword(QwordReg::Rax)),
                Token::Comma,
                Token::Reg(Reg::Qword(QwordReg::Rbx))
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
                Token::Reg(Reg::Qword(QwordReg::Rax)),
                Token::Comma,
                Token::Reg(Reg::Qword(QwordReg::Rbx)),
                Token::Opcode(Opcode::Xor),
                Token::Reg(Reg::Qword(QwordReg::Rbx)),
                Token::Comma,
                Token::Reg(Reg::Qword(QwordReg::Rax))
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
                Token::Reg(Reg::Qword(QwordReg::Rax)),
                Token::Comma,
                Token::Reg(Reg::Qword(QwordReg::Rax)),
                Token::Sublabel(Label(".if".into())),
                Token::Colon,
                Token::Opcode(Opcode::Add),
                Token::Reg(Reg::Qword(QwordReg::Rax)),
                Token::Comma,
                Token::Number(8),
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
    #[should_panic(expected = "invalid sublabel name")]
    fn invalid_sublabel() {
        let src = "..label:";

        let _ = lex(src);
    }
}
