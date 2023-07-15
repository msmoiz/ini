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

        while let Some(token) = self.lexer.peek() {
            match token {
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
        if let (Some(Token::LeftBracket), Some(Token::String(name)), Some(Token::RightBracket)) =
            (self.lexer.next(), self.lexer.next(), self.lexer.next())
        {
            return Ok(name);
        };
        Err(Error::Parse)
    }

    fn key(&mut self) -> Result<(String, String)> {
        if let (Some(Token::String(name)), Some(Token::Equal), Some(Token::String(value))) =
            (self.lexer.next(), self.lexer.next(), self.lexer.next())
        {
            if name.is_empty() {
                return Err(Error::Parse);
            }
            return Ok((name, value));
        }
        Err(Error::Parse)
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
}
