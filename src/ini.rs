use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use crate::parser::Parser;

/// INI section.
#[derive(Debug, PartialEq, Default)]
pub struct Section {
    /// Config keys, indexed by name.
    keys: HashMap<String, String>,
}

impl Section {
    /// Create a new Section.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a key.
    ///
    /// If a key exists with the same name, it is overwritten.
    pub fn insert(&mut self, name: String, value: String) {
        self.keys.insert(name, value);
    }
}

impl Index<&str> for Section {
    type Output = String;

    /// Returns a reference to the key with the specified name.
    ///
    /// Panics if there is no key with the specified name.
    fn index(&self, name: &str) -> &Self::Output {
        &self.keys[name]
    }
}

impl IndexMut<&str> for Section {
    /// Returns a mutable reference to the key with the specified name.
    ///
    /// Panics if there is no key with the specified name.
    fn index_mut(&mut self, name: &str) -> &mut Self::Output {
        let exp = format!("key {name} should exist");
        self.keys.get_mut(name).expect(&exp)
    }
}

/// INI config.
#[derive(Debug, PartialEq)]
pub struct Ini {
    /// Config sections, indexed by name.
    sections: HashMap<String, Section>,
}

impl Ini {
    // Create an Ini with a default section.
    pub fn new() -> Ini {
        let mut sections = HashMap::new();
        sections.insert("".into(), Section::new());
        Ini { sections }
    }

    /// Parse an Ini from an input string.
    pub fn from_str(text: &str) -> Ini {
        Parser::from_str(text)
    }

    /// Add an empty section.
    ///
    /// If a section with the specified name already exists, the original
    /// section will be discarded.
    pub fn add_section(&mut self, name: &str) {
        self.sections.insert(name.into(), Section::new());
    }

    /// Get a mutable section.
    ///
    /// If the section does not exist, this will panic.
    pub fn section_mut(&mut self, name: &str) -> &mut Section {
        self.sections.get_mut(name).unwrap()
    }
}

impl Index<&str> for Ini {
    type Output = Section;

    /// Returns a reference to the section with the specified name.
    ///
    /// Panics if there is no section with the specified name.
    fn index(&self, name: &str) -> &Self::Output {
        &self.sections[name]
    }
}

impl IndexMut<&str> for Ini {
    /// Returns a mutable reference to the section with the specified name.
    ///
    /// Panics if there is no section with the specified name.
    fn index_mut(&mut self, name: &str) -> &mut Self::Output {
        let exp = format!("section {name} should exist");
        self.sections.get_mut(name).expect(&exp)
    }
}
