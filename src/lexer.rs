#[derive(PartialEq, Debug)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Equal,
    Newline,
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

        if let Some(len) = self.scan_comment() {
            self.pos += len;
        }

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

        if let Some(len) = self.scan_newline() {
            self.pos += len;
            return Some(Newline);
        }

        let len = self.scan_string();
        {
            let string = &self.text[self.pos..self.pos + len];
            self.pos += len;
            return Some(String(string.into()));
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        let start_pos = self.pos;
        let token = self.next();
        self.pos = start_pos;
        token
    }

    fn skip_whitespace(&mut self) {
        let bytes = self.text.as_bytes();
        while self.pos < self.text.len() && matches!(bytes[self.pos], b' ' | b'\t') {
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

    fn scan_newline(&self) -> Option<usize> {
        assert!(self.pos < self.text.len());
        let current = self.text.as_bytes()[self.pos];
        if current == b'\n' {
            Some(1)
        } else if self.text[self.pos..].starts_with("\r\n") {
            Some(2)
        } else {
            None
        }
    }

    fn scan_comment(&self) -> Option<usize> {
        if self.pos >= self.text.len() {
            return None;
        }
        let bytes = self.text.as_bytes();
        let current = bytes[self.pos];
        if current == b';' {
            let mut ix = self.pos;
            let mut len = 0;
            while ix < self.text.len() {
                if bytes[ix] == b'\n'
                    || (bytes[ix] == b'\r' && ix + 1 < self.text.len() && bytes[ix + 1] == b'\n')
                {
                    break;
                }
                len += 1;
                ix += 1;
            }
            Some(len)
        } else {
            None
        }
    }

    fn scan_string(&self) -> usize {
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

        len
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
    fn multiple_tokens() {
        let text = "[]=";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Some(LeftBracket));
        assert_eq!(lexer.next(), Some(RightBracket));
        assert_eq!(lexer.next(), Some(Equal));
    }

    #[test]
    fn empty() {
        let text = "";
        let token = Lexer::new(text).next();
        assert!(token.is_none());
    }

    #[test]
    fn newline() {
        let text = "\n";
        let token = Lexer::new(text).next();
        assert_eq!(token, Some(Newline));
    }

    #[test]
    fn newline_win() {
        let text = "\r\nfoo";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Some(Newline));
        assert_eq!(lexer.next(), Some(String("foo".into())));
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

    #[test]
    fn standalone_comment() {
        let text = "; comment";
        let token = Lexer::new(text).next();
        assert!(token.is_none());
    }

    #[test]
    fn inline_comment() {
        let text = "
        [foo] ; comment
        bar=baz ; comment
        ";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Some(Newline));
        assert_eq!(lexer.next(), Some(LeftBracket));
        assert_eq!(lexer.next(), Some(String("foo".into())));
        assert_eq!(lexer.next(), Some(RightBracket));
        assert_eq!(lexer.next(), Some(Newline));
        assert_eq!(lexer.next(), Some(String("bar".into())));
        assert_eq!(lexer.next(), Some(Equal));
        assert_eq!(lexer.next(), Some(String("baz".into())));
        assert_eq!(lexer.next(), Some(Newline));
    }

    #[test]
    fn comment_win() {
        let text = "; comment\r\nfoo";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Some(Newline));
        assert_eq!(lexer.next(), Some(String("foo".into())));
    }
}
