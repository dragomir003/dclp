#![deny(missing_docs)]

//! This is a minimalist command line argument parser.

mod config;
mod arg;
mod parser;

pub use config::ConfigBuilder;
pub use parser::{ parse, ParseError };
pub use arg::{ Args, ParameterCount, args::* };