#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
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
    Identifier(String),
    StringLiteral(String),
    NumericLiteral(f64),

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

    Eof
}

pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
    pub column: u32
}

impl Token {
    pub fn new(token_type: TokenType, line: u32, column: u32) -> Token {
        Token{token_type: token_type, line: line, column: column}
    }
}
