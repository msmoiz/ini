//! This crate provides methods for working with INI files.
//!
//! The central type of this crate is `Ini`. It can be created in the following
//! ways:
//! * `Ini::new()` creates a new config object.
//! * `Ini::from_str()` parses a config object from an input string.
//!
//! The `Ini` object acts like a two-level hash map, with sections at the first
//! level and keys at the second level. It supports indexing operations for ease
//! of use.
//!
//! # Example
//!
//! The following example shows how to parse an `Ini` object from an input text.
//! The input contains one section (*greeting*) with two keys (*early* and
//! *late*).
//!
//! ```
//! use crate::ini::Ini;
//!
//! let ini = Ini::from_str("
//!     [greeting]
//!     early=morning
//!     late=night
//! ").unwrap();
//!
//! assert_eq!(ini["greeting"]["early"], "morning");
//! assert_eq!(ini["greeting"]["late"], "night");
//! ```
//!
//! # Syntax
//!
//! There is no standard specification for INI files, but this crate implements
//! a subset of the core features that appear common across most
//! implementations.
//!
//! ## Keys
//!
//! Individual settings are defined using keys, which are simple name-value
//! pairs delimited by an `=` sign. Keys may contain whitespace between elements
//! and must be followed by a newline or the end of file. Each key must appear
//! on its own line.
//!
//! ```ini
//! foo=bar # ok
//! foo=bar buz=bux # not ok
//! ```
//!
//! Names and values may be composed of ASCII alphanumeric characters and the
//! following symbols: `_./`. All other characters are disallowed. Internal
//! whitespace is disallowed as well.
//!
//! ```ini
//! foo=bar # ok
//! foo bar = baz bux # not ok
//! ```
//!
//! ## Sections
//!
//! Keys are grouped by section. A section ends when the next one begins or when
//! the end of file is reached.
//!
//! ```ini
//! [first]
//! foo=bar
//!
//! [second]
//! baz=bux
//! ```
//!
//! Keys declared before any section declaration are added to the default or
//! global section, which can be accessed with the name "".
//!
//! ```ini
//! foo=bar
//!
//! [first]
//! baz=bux
//! ```
//!
//! ## Comments
//!
//! Both Windows (`;`) and Unix style comments (`#`) are supported.
//!
//! ```ini
//! ; this is a comment
//! # so is this
//! ```
//!
//! Comments extend to the end of the line and all internal content is ignored.
//!
//! ```ini
//! ; foo=bar is not recognized
//! ```
//!
//! Comments may appear on separate lines or inline.
//!
//! ```ini
//! ; standalone comment
//! foo=bar ; inline comment
//! ```

mod error;
mod ini;
mod lexer;
mod parser;

pub use crate::ini::Ini;
