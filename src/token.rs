#[derive(Debug)]
pub enum Literal {
    Str(String),
    Num(f64),
    Nil,
    Bool(bool),
}

#[derive(Debug, Clone, Copy)]
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
