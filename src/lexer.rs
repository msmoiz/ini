use crate::error::{Error, Result};

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

    pub fn next(&mut self) -> Result<Option<Token>> {
        use Token::*;

        self.skip_whitespace();

        if let Some(len) = self.scan_comment() {
            self.pos += len;
        }

        if self.pos >= self.text.len() {
            return Ok(None);
        }

        if self.scan_left_bracket() {
            self.pos += 1;
            return Ok(Some(LeftBracket));
        }

        if self.scan_right_bracket() {
            self.pos += 1;
            return Ok(Some(RightBracket));
        }

        if self.scan_equal() {
            self.pos += 1;
            return Ok(Some(Equal));
        }

        if let Some(len) = self.scan_newline() {
            self.pos += len;
            return Ok(Some(Newline));
        }

        if let Some(len) = self.scan_quote_string()? {
            let string = self.text[self.pos + 1..self.pos + 1 + len].replace(r#"\""#, "\"");
            self.pos += len + 2;
            return Ok(Some(String(string)));
        }

        let len = self.scan_string();
        {
            let string = &self.text[self.pos..self.pos + len];
            self.pos += len;
            return Ok(Some(String(string.into())));
        }
    }

    pub fn peek(&mut self) -> Result<Option<Token>> {
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
        if current == b';' || current == b'#' {
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

    fn scan_quote_string(&self) -> Result<Option<usize>> {
        assert!(self.pos < self.text.len());
        let bytes = self.text.as_bytes();
        let mut ix = self.pos;
        if bytes[ix] != b'"' {
            return Ok(None);
        }
        ix += 1;
        let mut len = 0;
        while ix < self.text.len() {
            if bytes[ix] == b'"' {
                return Ok(Some(len));
            }
            if self.text[ix..].starts_with(r#"\""#) {
                ix += 2;
                len += 2;
                continue;
            }
            ix += 1;
            len += 1;
        }
        Err(Error::Parse)
    }

    fn scan_string(&self) -> usize {
        assert!(self.pos < self.text.len());
        let bytes = self.text.as_bytes();
        let mut ix = self.pos;
        let mut len = 0;

        while ix < self.text.len() {
            match bytes[ix] {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'.' | b'-' => {
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
    use crate::error::Result;

    #[test]
    fn left_bracket() {
        let text = "[";
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(LeftBracket));
    }

    #[test]
    fn right_bracket() {
        let text = "]";
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(RightBracket));
    }

    #[test]
    fn equals() {
        let text = "=";
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(Equal));
    }

    #[test]
    fn multiple_tokens() -> Result<()> {
        let text = "[]=";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next()?, Some(LeftBracket));
        assert_eq!(lexer.next()?, Some(RightBracket));
        assert_eq!(lexer.next()?, Some(Equal));
        Ok(())
    }

    #[test]
    fn empty() {
        let text = "";
        let token = Lexer::new(text).next().unwrap();
        assert!(token.is_none());
    }

    #[test]
    fn newline() {
        let text = "\n";
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(Newline));
    }

    #[test]
    fn newline_win() -> Result<()> {
        let text = "\r\nfoo";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next()?, Some(Newline));
        assert_eq!(lexer.next()?, Some(String("foo".into())));
        Ok(())
    }

    #[test]
    fn string() {
        let text = "hello";
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(String("hello".into())));
    }

    #[test]
    fn quote_string() {
        let text = r#""hello""#;
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(String("hello".into())));
    }

    #[test]
    fn escape_quote() {
        let text = r#""foo\"bar""#;
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(String("foo\"bar".into())));
    }

    #[test]
    fn mismatched_quote() {
        let text = r#""foo"#;
        let token = Lexer::new(text).next();
        assert!(token.is_err());
    }

    #[test]
    fn section() -> Result<()> {
        let text = "[section]";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next()?, Some(LeftBracket));
        assert_eq!(lexer.next()?, Some(String("section".into())));
        assert_eq!(lexer.next()?, Some(RightBracket));
        Ok(())
    }

    #[test]
    fn key() -> Result<()> {
        let text = "pi=3.14";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next()?, Some(String("pi".into())));
        assert_eq!(lexer.next()?, Some(Equal));
        assert_eq!(lexer.next()?, Some(String("3.14".into())));
        Ok(())
    }

    #[test]
    fn leading_whitespace() {
        let text = " foo";
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(String("foo".into())));
    }

    #[test]
    fn trailing_whitespace() {
        let text = "foo ";
        let token = Lexer::new(text).next().unwrap();
        assert_eq!(token, Some(String("foo".into())));
    }

    #[test]
    fn standalone_comment() {
        let text = "; comment";
        let token = Lexer::new(text).next().unwrap();
        assert!(token.is_none());
    }

    #[test]
    fn inline_comment() -> Result<()> {
        let text = "
        [foo] ; comment
        bar=baz ; comment
        ";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next()?, Some(Newline));
        assert_eq!(lexer.next()?, Some(LeftBracket));
        assert_eq!(lexer.next()?, Some(String("foo".into())));
        assert_eq!(lexer.next()?, Some(RightBracket));
        assert_eq!(lexer.next()?, Some(Newline));
        assert_eq!(lexer.next()?, Some(String("bar".into())));
        assert_eq!(lexer.next()?, Some(Equal));
        assert_eq!(lexer.next()?, Some(String("baz".into())));
        assert_eq!(lexer.next()?, Some(Newline));
        Ok(())
    }

    #[test]
    fn comment_win() -> Result<()> {
        let text = "; comment\r\nfoo";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next()?, Some(Newline));
        assert_eq!(lexer.next()?, Some(String("foo".into())));
        Ok(())
    }

    #[test]
    fn comment_unix_style() -> Result<()> {
        let text = "# comment\nfoo";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next()?, Some(Newline));
        assert_eq!(lexer.next()?, Some(String("foo".into())));
        Ok(())
    }
}
