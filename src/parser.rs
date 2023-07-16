use crate::{
    error::Error,
    lexer::{Lexer, Token},
    Ini,
};

use crate::error::Result;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn from_str(text: &str) -> Result<Ini> {
        let lexer = Lexer::new(text);
        let mut parser = Parser { lexer };
        parser.ini()
    }

    fn ini(&mut self) -> Result<Ini> {
        let mut ini = Ini::new();
        let mut cur_section = "".to_string();

        while let Some(token) = self.lexer.peek()? {
            match token {
                Token::Newline => {
                    self.lexer.next()?;
                    continue;
                }
                Token::LeftBracket => {
                    let name = self.section()?;
                    ini.add_section(&name);
                    cur_section = name;
                }
                Token::String(_) => {
                    let (name, value) = self.key()?;
                    ini[&cur_section].insert(name, value);
                }
                _ => return Err(Error::Parse),
            }
        }

        Ok(ini)
    }

    fn section(&mut self) -> Result<String> {
        let left_br = self.lexer.next()?;
        let name = self.lexer.next()?;
        let right_br = self.lexer.next()?;
        let newline = self.lexer.next()?;
        match (left_br, name, right_br, newline) {
            (
                Some(Token::LeftBracket),
                Some(Token::String(name)),
                Some(Token::RightBracket),
                Some(Token::Newline),
            )
            | (
                Some(Token::LeftBracket),
                Some(Token::String(name)),
                Some(Token::RightBracket),
                None,
            ) => Ok(name),
            _ => Err(Error::Parse),
        }
    }

    fn key(&mut self) -> Result<(String, String)> {
        let name = self.lexer.next()?;
        let equal = self.lexer.next()?;
        let value = self.lexer.next()?;
        let newline = self.lexer.next()?;
        match (name, equal, value, newline) {
            (
                Some(Token::String(name)),
                Some(Token::Equal),
                Some(Token::String(value)),
                Some(Token::Newline),
            )
            | (Some(Token::String(name)), Some(Token::Equal), Some(Token::String(value)), None) => {
                if name.is_empty() {
                    return Err(Error::Parse);
                }
                Ok((name, value))
            }
            _ => Err(Error::Parse),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_section() {
        let text = "";
        let ini = Parser::from_str(text);
        let expected = Ini::new();
        assert_eq!(ini, Ok(expected));
    }

    #[test]
    fn default_section_key() {
        let text = "bar=baz";
        let ini = Parser::from_str(text);
        let mut expected = Ini::new();
        expected[""].insert("bar".into(), "baz".into());
        assert_eq!(ini, Ok(expected));
    }

    #[test]
    fn section() {
        let text = "[foo]";
        let ini = Parser::from_str(text);
        let mut expected = Ini::new();
        expected.add_section("foo");
        assert_eq!(ini, Ok(expected));
    }

    #[test]
    fn section_key() {
        let text = r"
        [foo]
        bar=baz
        ";
        let ini = Parser::from_str(text);
        let mut expected = Ini::new();
        expected.add_section("foo");
        expected["foo"].insert("bar".into(), "baz".into());
        assert_eq!(ini, Ok(expected));
    }

    #[test]
    fn many_sections() {
        let text = r"
        [foo]
        [bar]
        [baz]
        ";
        let ini = Parser::from_str(text);
        let mut expected = Ini::new();
        expected.add_section("foo");
        expected.add_section("bar");
        expected.add_section("baz");
        assert_eq!(ini, Ok(expected));
    }

    #[test]
    fn keys_on_same_line() {
        let text = "bar=baz qux=quux";
        let ini = Parser::from_str(text);
        assert!(ini.is_err());
    }

    #[test]
    fn sections_on_same_line() {
        let text = "[foo] [bar]";
        let ini = Parser::from_str(text);
        assert!(ini.is_err());
    }

    #[test]
    fn section_key_on_same_line() {
        let text = "[foo] bar=baz";
        let ini = Parser::from_str(text);
        assert!(ini.is_err());
    }

    #[test]
    fn key_split_across_line() {
        let text = "bar=\nbaz";
        let ini = Parser::from_str(text);
        assert!(ini.is_err());
    }

    #[test]
    fn section_split_across_line() {
        let text = "[foo\n]";
        let ini = Parser::from_str(text);
        assert!(ini.is_err());
    }

    #[test]
    fn section_quoted_name() {
        let text = r#"["foo bar"]"#;
        let ini = Parser::from_str(text);
        let mut expected = Ini::new();
        expected.add_section("foo bar");
        assert_eq!(ini, Ok(expected));
    }

    #[test]
    fn key_quoted_name() {
        let text = r#""foo bar"=baz"#;
        let ini = Parser::from_str(text).unwrap();
        assert_eq!(ini[""]["foo bar"], "baz");
    }

    #[test]
    fn key_quoted_value() {
        let text = r#"foo="bar baz""#;
        let ini = Parser::from_str(text).unwrap();
        assert_eq!(ini[""]["foo"], "bar baz");
    }
}
