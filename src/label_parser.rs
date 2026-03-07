use std::{collections::HashMap, iter::Peekable, slice::Iter};

use crate::{operands::Label, tokens::Token};

pub struct LabelParser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    parent_label: Label,
    current_instr_idx: usize,
    labels: HashMap<Label, usize>,
}

impl<'a> LabelParser<'a> {
    pub fn new(tokens: Iter<'a, Token>) -> Self {
        Self {
            tokens: tokens.peekable(),
            parent_label: Label(String::new()),
            current_instr_idx: 0,
            labels: HashMap::new(),
        }
    }

    /// Gets all labels in the source text.
    pub fn parse(mut self) -> HashMap<Label, usize> {
        while let Some(token) = self.next() {
            match token {
                Token::Label(label) if self.peek() == Some(&Token::Colon) => {
                    self.parse_label(label);
                }
                Token::Sublabel(label) if self.peek() == Some(&Token::Colon) => {
                    self.parse_sublabel(label);
                }
                Token::Opcode(_) => self.current_instr_idx += 1,
                _ => {}
            }
        }

        self.labels
    }

    fn parse_label(&mut self, label: &Label) {
        self.parent_label.clone_from(label);
        if let Some(idx) = self.labels.get(label)
            && *idx != self.current_instr_idx
        {
            panic!("inconsistent redefinition of {}", label.0);
        }

        self.labels.insert(label.clone(), self.current_instr_idx);
    }

    fn parse_sublabel(&mut self, label: &Label) {
        let label = Label(self.parent_label.0.clone() + &label.0);
        if let Some(idx) = self.labels.get(&label)
            && *idx != self.current_instr_idx
        {
            panic!("inconsistent redefinition of {}", label.0);
        }

        self.labels.insert(label.clone(), self.current_instr_idx);
    }

    fn next(&mut self) -> Option<&'a Token> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&'a Token> {
        self.tokens.peek().copied()
    }
}

pub fn fix_opcode_label_definitions(tokens: &mut [Token]) {
    for i in 0..tokens.len() {
        match tokens[i] {
            Token::Opcode(op) if tokens.get(i + 1) == Some(&Token::Colon) => {
                tokens[i] = Token::Label(Label(op.as_str().to_string()));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn fixing() {
        let src = "
    mov:
    .add:
";
        let mut tokens = Lexer::new(src).lex();
        fix_opcode_label_definitions(&mut tokens);
        assert_eq!(
            tokens,
            vec![
                Token::Label(Label("mov".into())),
                Token::Colon,
                Token::Sublabel(Label(".add".into())),
                Token::Colon
            ]
        );
    }

    #[test]
    fn label() {
        let src = "
    label:
    .sub:
    label:
        xor rax, rax
";

        let mut tokens = Lexer::new(src).lex();
        fix_opcode_label_definitions(&mut tokens);
        let result = LabelParser::new(tokens.iter()).parse();
        let mut expected = HashMap::new();
        expected.insert(Label("label".into()), 0);
        expected.insert(Label("label.sub".into()), 0);
        assert_eq!(expected, result);
    }

    #[test]
    fn opcode_label() {
        let src = "
    sub:
    add:
        xor rax, rax
";
        let mut tokens = Lexer::new(src).lex();
        fix_opcode_label_definitions(&mut tokens);
        let result = LabelParser::new(tokens.iter()).parse();
        let mut expected = HashMap::new();
        expected.insert(Label("sub".into()), 0);
        expected.insert(Label("add".into()), 0);
        assert_eq!(expected, result);
    }

    #[test]
    fn jmp() {
        let src = "
    label:
        jmp .sub
    .sub:
        jmp label
    label2:
";

        let mut tokens = Lexer::new(src).lex();
        fix_opcode_label_definitions(&mut tokens);
        let result = LabelParser::new(tokens.iter()).parse();
        let mut expected = HashMap::new();
        expected.insert(Label("label".into()), 0);
        expected.insert(Label("label.sub".into()), 1);
        expected.insert(Label("label2".into()), 2);
        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic(expected = "inconsistent redefinition of label")]
    fn inconsistent_redefinition() {
        let src = "
    label:
        jmp .sub
    label:
        jmp label
";

        let mut tokens = Lexer::new(src).lex();
        fix_opcode_label_definitions(&mut tokens);
        _ = LabelParser::new(tokens.iter()).parse();
    }

    #[test]
    fn single_sublabel() {
        let src = "
    .label:
";

        let mut tokens = Lexer::new(src).lex();
        fix_opcode_label_definitions(&mut tokens);
        let result = LabelParser::new(tokens.iter()).parse();
        let mut expected = HashMap::new();
        expected.insert(Label(".label".into()), 0);
        assert_eq!(expected, result);
    }
}
