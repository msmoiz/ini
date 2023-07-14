use std::collections::HashMap;

use crate::parser::Parser;

#[derive(Debug, PartialEq, Default)]
pub struct Section {
    keys: HashMap<String, String>,
}

impl Section {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_key(&mut self, name: String, value: String) {
        self.keys.insert(name, value);
    }
}

#[derive(Debug, PartialEq)]
pub struct Ini {
    sections: HashMap<String, Section>,
}

impl Ini {
    pub fn new() -> Ini {
        let mut sections = HashMap::new();
        sections.insert("".into(), Section::new());
        Ini { sections }
    }

    pub fn from_str(text: &str) -> Ini {
        Parser::from_str(text)
    }

    pub fn add_section(&mut self, name: &str) {
        self.sections.insert(name.into(), Section::new());
    }

    pub fn section_mut(&mut self, name: &str) -> &mut Section {
        self.sections.get_mut(name).unwrap()
    }
}
