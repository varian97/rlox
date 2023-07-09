use crate::error::RloxErrorDetail;

#[derive(Debug)]
pub enum Literal {
    Str(String),
    Num(f64),
    Nil,
    Bool(bool),
}

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line_number: usize,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line_number: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            line_number,
            literal: match literal {
                Some(literal) => Some(literal),
                None => None,
            },
        }
    }

    pub fn eof(line_number: usize) -> Self {
        Token {
            token_type: TokenType::Eof,
            lexeme: String::from(""),
            line_number,
            literal: None,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

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
            _ => {
                return Err(RloxErrorDetail::new(
                    self.line,
                    "Unexpected Token".to_string(),
                ))
            }
        }?;

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let res = self.source[self.current];
        self.current += 1;
        res as char
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
