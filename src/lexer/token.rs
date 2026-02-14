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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

impl core::fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {})",
            self.token_type, self.lexeme, self.line
        )
    }
}

impl core::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_debug() {
        let token = Token::new(TokenType::Identifier, "foo", 1);
        let debug_output = format!("{:?}", token);
        assert_eq!(debug_output, r#"(Identifier, "foo", 1)"#);
    }
}
