/// Error type for INI operations.
#[derive(PartialEq, Debug)]
pub enum Error {
    Parse,
}

/// Result type for INI operations.
pub type Result<T> = std::result::Result<T, Error>;
