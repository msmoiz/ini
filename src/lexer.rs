#[derive(PartialEq, Debug)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Equal,
    Semicolon,
    String(String),
}

pub struct Lexer<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &str) -> Lexer {
        Lexer { text, pos: 0 }
    }

    pub fn next(&mut self) -> Option<Token> {
        use Token::*;

        self.skip_whitespace();

        if self.pos >= self.text.len() {
            return None;
        }

        if self.scan_left_bracket() {
            self.pos += 1;
            return Some(LeftBracket);
        }

        if self.scan_right_bracket() {
            self.pos += 1;
            return Some(RightBracket);
        }

        if self.scan_equal() {
            self.pos += 1;
            return Some(Equal);
        }

        if self.scan_semicolon() {
            self.pos += 1;
            return Some(Semicolon);
        }

        if let Some(len) = self.scan_string() {
            let string = &self.text[self.pos..self.pos + len];
            self.pos += len;
            return Some(String(string.into()));
        }

        unreachable!("string is a catchall token")
    }

    fn skip_whitespace(&mut self) {
        let bytes = self.text.as_bytes();
        while self.pos < self.text.len() && matches!(bytes[self.pos], b' ' | b'\t' | b'\n' | b'\r')
        {
            self.pos += 1;
        }
    }

    fn scan_left_bracket(&self) -> bool {
        assert!(self.pos < self.text.len());
        let current = self.text.as_bytes()[self.pos];
        current == b'['
    }

    fn scan_right_bracket(&self) -> bool {
        assert!(self.pos < self.text.len());
        let current = self.text.as_bytes()[self.pos];
        current == b']'
    }

    fn scan_equal(&self) -> bool {
        assert!(self.pos < self.text.len());
        let current = self.text.as_bytes()[self.pos];
        current == b'='
    }

    fn scan_semicolon(&self) -> bool {
        assert!(self.pos < self.text.len());
        let current = self.text.as_bytes()[self.pos];
        current == b';'
    }

    fn scan_string(&self) -> Option<usize> {
        assert!(self.pos < self.text.len());
        let bytes = self.text.as_bytes();
        let mut ix = self.pos;
        let mut len = 0;

        while ix < self.text.len() {
            match bytes[ix] {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'.' | b'/' => {
                    len += 1;
                    ix += 1;
                }
                _ => break,
            }
        }

        if len == 0 {
            return None;
        }

        Some(len)
    }
}

#[cfg(test)]
mod tests {
    use super::{Token::*, *};

    #[test]
    fn left_bracket() {
        let text = "[";
        let token = Lexer::new(text).next();
        assert_eq!(token, Some(LeftBracket));
    }

    #[test]
    fn right_bracket() {
        let text = "]";
        let token = Lexer::new(text).next();
        assert_eq!(token, Some(RightBracket));
    }

    #[test]
    fn equals() {
        let text = "=";
        let token = Lexer::new(text).next();
        assert_eq!(token, Some(Equal));
    }

    #[test]
    fn semicolon() {
        let text = ";";
        let token = Lexer::new(text).next();
        assert_eq!(token, Some(Semicolon));
    }

    #[test]
    fn multiple_tokens() {
        let text = "[]=;";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Some(LeftBracket));
        assert_eq!(lexer.next(), Some(RightBracket));
        assert_eq!(lexer.next(), Some(Equal));
        assert_eq!(lexer.next(), Some(Semicolon));
    }

    #[test]
    fn empty() {
        let text = "";
        let token = Lexer::new(text).next();
        assert!(token.is_none());
    }

    #[test]
    fn string() {
        let text = "hello";
        let token = Lexer::new(text).next();
        assert_eq!(token, Some(String("hello".into())));
    }

    #[test]
    fn section() {
        let text = "[section]";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Some(LeftBracket));
        assert_eq!(lexer.next(), Some(String("section".into())));
        assert_eq!(lexer.next(), Some(RightBracket));
    }

    #[test]
    fn key() {
        let text = "pi=3.14";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Some(String("pi".into())));
        assert_eq!(lexer.next(), Some(Equal));
        assert_eq!(lexer.next(), Some(String("3.14".into())));
    }

    #[test]
    fn leading_whitespace() {
        let text = " foo";
        let token = Lexer::new(text).next();
        assert_eq!(token, Some(String("foo".into())));
    }

    #[test]
    fn trailing_whitespace() {
        let text = "foo ";
        let token = Lexer::new(text).next();
        assert_eq!(token, Some(String("foo".into())));
    }
}
