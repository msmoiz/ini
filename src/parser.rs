use crate::{
    lexer::{Lexer, Token},
    Ini,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn from_str(text: &str) -> Ini {
        let lexer = Lexer::new(text);
        let mut parser = Parser { lexer };
        parser.ini()
    }

    fn ini(&mut self) -> Ini {
        let mut ini = Ini::new();
        let mut cur_section = "".to_string();

        while let Some(token) = self.lexer.peek() {
            match token {
                Token::LeftBracket => {
                    if let Some(name) = self.section() {
                        ini.add_section(&name);
                        cur_section = name;
                    } else {
                        todo!()
                    }
                }
                Token::String(_) => {
                    if let Some((name, value)) = self.key() {
                        ini[&cur_section].insert(name, value);
                    }
                }
                _ => todo!(),
            }
        }

        ini
    }

    fn section(&mut self) -> Option<String> {
        match (self.lexer.next(), self.lexer.next(), self.lexer.next()) {
            (Some(Token::LeftBracket), Some(Token::String(name)), Some(Token::RightBracket)) => {
                Some(name)
            }
            _ => todo!(),
        }
    }

    fn key(&mut self) -> Option<(String, String)> {
        match (self.lexer.next(), self.lexer.next(), self.lexer.next()) {
            (Some(Token::String(name)), Some(Token::Equal), Some(Token::String(value))) => {
                if name.is_empty() {
                    todo!()
                }
                Some((name, value))
            }
            _ => todo!(),
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
        assert_eq!(ini, expected);
    }

    #[test]
    fn default_section_key() {
        let text = "bar=baz";
        let ini = Parser::from_str(text);
        let mut expected = Ini::new();
        expected[""].insert("bar".into(), "baz".into());
        assert_eq!(ini, expected);
    }

    #[test]
    fn section() {
        let text = "[foo]";
        let ini = Parser::from_str(text);
        let mut expected = Ini::new();
        expected.add_section("foo");
        assert_eq!(ini, expected);
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
        assert_eq!(ini, expected);
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
        assert_eq!(ini, expected);
    }
}
