use crate::error::RloxErrorDetail;
use crate::token::{Token, TokenType, Literal};

use phf::phf_map;

static TOKEN_TYPE_BY_RESERVED_KEYWORD: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

pub struct Scanner {
    pub source: Vec<u8>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.into_bytes(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, RloxErrorDetail> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), RloxErrorDetail> {
        let c = self.advance();

        match c {
            '(' => self.add_token_without_literal(TokenType::LeftParen),
            ')' => self.add_token_without_literal(TokenType::RightParen),
            '{' => self.add_token_without_literal(TokenType::LeftBrace),
            '}' => self.add_token_without_literal(TokenType::RightBrace),
            ',' => self.add_token_without_literal(TokenType::Comma),
            '.' => self.add_token_without_literal(TokenType::Dot),
            '-' => self.add_token_without_literal(TokenType::Minus),
            '+' => self.add_token_without_literal(TokenType::Plus),
            ';' => self.add_token_without_literal(TokenType::Semicolon),
            '*' => self.add_token_without_literal(TokenType::Star),
            '!' => {
                let match_next = self.is_next_char_match('=');
                if match_next {
                    self.add_token_without_literal(TokenType::BangEqual)
                } else {
                    self.add_token_without_literal(TokenType::Bang)
                }
            }
            '=' => {
                let match_next = self.is_next_char_match('=');
                if match_next {
                    self.add_token_without_literal(TokenType::EqualEqual)
                } else {
                    self.add_token_without_literal(TokenType::Equal)
                }
            }
            '<' => {
                let match_next = self.is_next_char_match('=');
                if match_next {
                    self.add_token_without_literal(TokenType::LessEqual)
                } else {
                    self.add_token_without_literal(TokenType::Less)
                }
            }
            '>' => {
                let match_next = self.is_next_char_match('=');
                if match_next {
                    self.add_token_without_literal(TokenType::GreaterEqual)
                } else {
                    self.add_token_without_literal(TokenType::Greater)
                }
            }
            '/' => {
                let match_comment = self.is_next_char_match('/');
                if match_comment {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token_without_literal(TokenType::Slash)?;
                }
                Ok(())
            }
            '\n' => {
                self.line += 1;
                Ok(())
            }
            ' ' | '\r' | '\t' => Ok(()),
            '"' => self.add_string_token(),
            other => {
                if self.is_digit(other) {
                    return self.add_number_token();
                }

                if self.is_alpha(other) {
                    return self.add_identifier_token();
                }

                return Err(RloxErrorDetail::new(
                    self.line,
                    "Unexpected Token".to_string(),
                ));
            }
        }?;

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(&self, ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn is_alpha(&self, ch: char) -> bool {
        (ch >= 'A' && ch <= 'Z') || (ch >= 'a' && ch <= 'z') || ch == '_'
    }

    fn is_alphanumeric(&self, ch: char) -> bool {
        self.is_alpha(ch) || self.is_digit(ch)
    }

    fn advance(&mut self) -> char {
        let res = self.source[self.current];
        self.current += 1;
        res as char
    }

    fn substring(&self, start: usize, end: usize) -> Result<String, RloxErrorDetail> {
        let buf = &self.source[start..end];

        let text = match std::str::from_utf8(buf) {
            Ok(s) => s.to_string(),
            Err(_) => {
                return Err(RloxErrorDetail::new(
                    self.line,
                    "Invalid UTF-8 sequence".to_string(),
                ));
            }
        };

        Ok(text)
    }

    fn is_next_char_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current] as char
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() || self.current == self.source.len() - 1 {
            return '\0';
        }
        self.source[self.current + 1] as char
    }

    fn add_identifier_token(&mut self) -> Result<(), RloxErrorDetail> {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = self.substring(self.start, self.current)?;
        let token_type = match TOKEN_TYPE_BY_RESERVED_KEYWORD.get(&text).copied() {
            Some(_token_type) => _token_type,
            None => TokenType::Identifier,
        };

        self.add_token_without_literal(token_type)
    }

    fn add_string_token(&mut self) -> Result<(), RloxErrorDetail> {
        while !self.is_at_end() && self.peek() != '"' {
            let curr = self.peek();
            if curr == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(RloxErrorDetail {
                line_number: self.line,
                message: String::from("Unterminated String"),
            });
        }

        // consume the closing "
        self.advance();

        let str_value = self.substring(self.start + 1, self.current - 1)?;

        self.add_token_object(TokenType::String, Some(Literal::Str(str_value)))
    }

    fn add_number_token(&mut self) -> Result<(), RloxErrorDetail> {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let number_value = self.substring(self.start, self.current)?;

        self.add_token_object(
            TokenType::Number,
            Some(Literal::Num(number_value.parse().unwrap())),
        )
    }

    fn add_token_without_literal(&mut self, token_type: TokenType) -> Result<(), RloxErrorDetail> {
        self.add_token_object(token_type, None)?;
        Ok(())
    }

    fn add_token_object(
        &mut self,
        token_type: TokenType,
        literal: Option<Literal>,
    ) -> Result<(), RloxErrorDetail> {
        let buf = &self.source[self.start..self.current];

        let text = match std::str::from_utf8(buf) {
            Ok(s) => s.to_string(),
            Err(_) => {
                return Err(RloxErrorDetail::new(
                    self.line,
                    "Invalid UTF-8 sequence".to_string(),
                ));
            }
        };

        self.tokens
            .push(Token::new(token_type, text, literal, self.line));

        Ok(())
    }
}
