use super::token::Token;
use super::token::TokenType;

pub struct Lexer<'a> {
    source: &'a str,
    start: usize,
    curr: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            curr: 0,
            line: 1,
        }
    }

    pub fn advance(&mut self) -> char {
        let ch = self.source.as_bytes()[self.curr] as char;
        self.curr += 1;
        ch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance_single_character() {
        let source = "abc";
        let mut lexer = Lexer::new(source);

        let ch = lexer.advance();

        assert_eq!(ch, 'a');
        assert_eq!(lexer.curr, 1);
    }

    #[test]
    fn test_advance_multiple_characters() {
        let source = "abc";
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.advance(), 'a');
        assert_eq!(lexer.advance(), 'b');
        assert_eq!(lexer.advance(), 'c');
        assert_eq!(lexer.curr, 3);
    }
}
