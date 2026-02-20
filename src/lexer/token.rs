#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens
    LParen,
    RParen,
    LCurly,
    RCurly,
    LSquare,
    RSquare,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Mod,
    Colon,
    Semicolon,
    Question,
    Not,
    Gt,
    Lt,

    // Two-character tokens
    Ge,
    Le,
    Ne,
    Eq,
    Assign,
    GtGt,
    LtLt,

    // Literals
    Identifier,
    String,
    Integer,
    Float,

    // Keywords
    If,
    Then,
    Else,
    True,
    False,
    And,
    Or,
    While,
    Do,
    For,
    Func,
    Null,
    End,
    Print,
    Println,
    Ret,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}
