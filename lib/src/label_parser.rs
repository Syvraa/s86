use std::{collections::HashMap, iter::Peekable, slice::Iter};

use crate::{
    operands::Label,
    syntax_error::{SyntaxError, SyntaxErrorKind},
    tokens::{Token, TokenType},
};

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

    /// Gets all label definitions in the source text.
    pub fn parse(mut self) -> Result<HashMap<Label, usize>, Vec<SyntaxError>> {
        let mut errors = Vec::new();
        while let Some(token) = self.next() {
            match &token.ty {
                TokenType::Label(label)
                    if self.peek().is_some_and(|t| t.ty == TokenType::Colon) =>
                {
                    if let Err(error) = self.insert_label(label, token.line) {
                        errors.push(error);
                    }
                }
                TokenType::Sublabel(label)
                    if self.peek().is_some_and(|t| t.ty == TokenType::Colon) =>
                {
                    if let Err(error) = self.insert_sublabel(label, token.line) {
                        errors.push(error);
                    }
                }
                TokenType::Opcode(_) => self.current_instr_idx += 1,
                _ => {}
            }
        }

        if errors.is_empty() {
            Ok(self.labels)
        } else {
            Err(errors)
        }
    }

    fn insert_label(&mut self, label: &Label, line: usize) -> Result<(), SyntaxError> {
        self.parent_label.clone_from(label);
        if let Some(idx) = self.labels.get(label)
            && *idx != self.current_instr_idx
        {
            return Err(SyntaxError {
                line,
                error: SyntaxErrorKind::InconsistentLabelRedefinition,
            });
        }
        self.labels.insert(label.clone(), self.current_instr_idx);

        Ok(())
    }

    fn insert_sublabel(&mut self, label: &Label, line: usize) -> Result<(), SyntaxError> {
        let label_text = self.parent_label.0.clone() + &label.0;
        if let Some(idx) = self.labels.get(label)
            && *idx != self.current_instr_idx
        {
            return Err(SyntaxError {
                line,
                error: SyntaxErrorKind::InconsistentLabelRedefinition,
            });
        }

        self.labels
            .insert(Label(label_text), self.current_instr_idx);

        Ok(())
    }

    fn next(&mut self) -> Option<&'a Token> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&'a Token> {
        self.tokens.peek().copied()
    }
}

/// By default, labels like mov: are parsed as an opcode. This function makes those into labels.
pub fn fix_opcode_label_definitions(tokens: &mut [Token]) {
    for i in 0..tokens.len() {
        let token = &tokens[i];
        match token.ty {
            TokenType::Opcode(op)
                if tokens.get(i + 1).is_some_and(|t| t.ty == TokenType::Colon) =>
            {
                tokens[i] = Token {
                    ty: TokenType::Label(Label(op.as_str().to_string())),
                    ..tokens[i]
                };
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
        let src = "mov:
.add:";
        let mut tokens = Lexer::new(src).lex().unwrap();
        fix_opcode_label_definitions(&mut tokens);
        assert_eq!(
            tokens,
            vec![
                Token {
                    line: 1,
                    ty: TokenType::Label(Label("mov".into()))
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
                    ty: TokenType::Sublabel(Label(".add".into()))
                },
                Token {
                    line: 2,
                    ty: TokenType::Colon
                },
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

        let mut tokens = Lexer::new(src).lex().unwrap();
        fix_opcode_label_definitions(&mut tokens);
        let result = LabelParser::new(tokens.iter()).parse().unwrap();
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
        let mut tokens = Lexer::new(src).lex().unwrap();
        fix_opcode_label_definitions(&mut tokens);
        let result = LabelParser::new(tokens.iter()).parse().unwrap();
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

        let mut tokens = Lexer::new(src).lex().unwrap();
        fix_opcode_label_definitions(&mut tokens);
        let result = LabelParser::new(tokens.iter()).parse().unwrap();
        let mut expected = HashMap::new();
        expected.insert(Label("label".into()), 0);
        expected.insert(Label("label.sub".into()), 1);
        expected.insert(Label("label2".into()), 2);
        assert_eq!(expected, result);
    }

    #[test]
    fn inconsistent_redefinition() {
        let src = "label:
jmp .sub
label:
jmp label
";

        let mut tokens = Lexer::new(src).lex().unwrap();
        fix_opcode_label_definitions(&mut tokens);
        let errors = LabelParser::new(tokens.iter()).parse().unwrap_err();
        assert_eq!(
            errors,
            vec![SyntaxError {
                line: 3,
                error: SyntaxErrorKind::InconsistentLabelRedefinition
            }]
        );
    }

    #[test]
    fn single_sublabel() {
        let src = "
    .label:
";

        let mut tokens = Lexer::new(src).lex().unwrap();
        fix_opcode_label_definitions(&mut tokens);
        let result = LabelParser::new(tokens.iter()).parse().unwrap();
        let mut expected = HashMap::new();
        expected.insert(Label(".label".into()), 0);
        assert_eq!(expected, result);
    }
}
