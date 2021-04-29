use std::fmt;
use std::error::Error;

/// Represents an Error that happpens during parsing
#[derive(Debug)]
pub enum ParseError {
    /// Constructed when unknown option is encountered
    InvalidOption(String),
    /// Constructed when unknown subcommand is encountered
    InvalidCommand(String),
    /// Constructed when the number of parameters is not satisfied
    InvalidNumberOfParameters(String),
    /// Constructed when there are no CLI arguments
    NoProgramName,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidOption(s) => write!(f, "Invalid option encountered: {}", s),
            ParseError::InvalidCommand(s) => write!(f, "Invalid subcommand encountered: {}", s),
            ParseError::InvalidNumberOfParameters(s) => write!(f, "The number of parameters constraint is not satisfied: {}", s),
            ParseError::NoProgramName => write!(f, "There were no command line arguments."),
        }
    }
}

impl Error for ParseError {}