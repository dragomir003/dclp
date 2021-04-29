pub mod args;

use std::collections::HashMap;
use std::fmt;

/// Represents a CLI Argument
#[derive(Debug, Default)]
pub struct Arg {
    /// Short name of an argument(will not be used if kind is ```ArgType::Subcommand```)
    pub short: Option<char>,
    /// Long name of an argument(will not be used if kind is ```ArgType::Subcommand```, instead name will be used)
    pub long: Option<String>,
    /// Name thet is used internally to refer to this exact instance of argument
    pub name: String,
    /// Type of argument
    pub kind: ArgType,
    /// Number of parameters argument takes
    pub parameter_count: ParameterCount,
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.short.is_some() && self.long.is_some() {
            write!(f, "(Argument {} [short: -{}, long: --{} ] )", self.name, self.short.unwrap(), self.long.as_ref().unwrap())
        } else if self.short.is_some() {
            write!(f, "(Argument {}[short: -{}])", self.name, self.short.unwrap())
        } else if self.long.is_some() {
            write!(f, "(Argument {} [long: --{} ] )", self.name, self.long.as_ref().unwrap())
        } else {
            write!(f, "(Argument {})", self.name)
        }
    }
}

/// Represents every possible variation for the amount of Parameters
#[derive(Debug, Clone, Copy)]
pub enum ParameterCount {
    /// This option or subcommand has 0 arguments
    Zero,
    /// This option or subcommand has more than n arguments
    More(usize),
    /// This option or subcommand has less than n arguments
    Less(usize),
    /// This option or subcommand has exactly n arguments
    Exact(usize),
}

impl Default for ParameterCount {
    fn default() -> Self {
        ParameterCount::Zero
    }
}

/// Represent a type of an Argument
#[derive(Debug, Eq, PartialEq)]
pub enum ArgType {
    /// Does not have '-' or '--' in front of an argument
    Subcommand,
    /// Includes '-' or '--' in front of an argument
    Option,
}

impl Default for ArgType {
    fn default() -> Self {
        ArgType::Option
    }
}

/// Represents final representation of CL Arguments
/// Key to this HashMap is a name originally assigned to ```Arg``` and
/// and value is an ```Option<Vec<_>>``` which holds parameters to that arguments
/// # Note
/// If value is ```None``` that means the argument has not appeared
///
/// Also, there is a special value with name '{program}' which holds parameters that were
/// not attributed to any other flag or subcommand
pub type Args = HashMap<String, Option<Vec<String>>>;