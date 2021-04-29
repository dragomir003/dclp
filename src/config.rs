use crate::arg::{ Arg, ArgType, ParameterCount };

/// Builds Config
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    args: Vec<Arg>,
}

impl ConfigBuilder {
    /// Adds an option which only has a short version
    pub fn add_short_option(mut self, name: String, short: char, parameter_count: ParameterCount) -> Self {
        self.args.push(Arg {
            short: Some(short),
            long: None,
            name,
            kind: ArgType::Option,
            parameter_count,
        });

        self
    }

    /// Adds an option which only has a long version
    pub fn add_long_option(mut self, name: String, long: String, parameter_count: ParameterCount) -> Self {
        self.args.push(Arg {
            short: None,
            long: Some(long),
            name,
            kind: ArgType::Option,
            parameter_count,
        });

        self
    }

    /// Adds an option
    pub fn add_option(mut self, name: String, short: char, long: String, parameter_count: ParameterCount) -> Self {
        self.args.push(Arg {
            short: Some(short),
            long: Some(long),
            name,
            kind: ArgType::Option,
            parameter_count,
        });

        self
    }

    /// Adds a flag
    pub fn add_flag(mut self, name: String, short: char, long: String) -> Self {
        self.args.push(Arg {
            short: Some(short),
            long: Some(long),
            name,
            kind: ArgType::Option,
            parameter_count: ParameterCount::Zero,
        });

        self
    }

    /// Adds a flag which only has a short version
    pub fn add_short_flag(mut self, name: String, short: char) -> Self {
        self.args.push(Arg {
            short: Some(short),
            long: None,
            name,
            kind: ArgType::Option,
            parameter_count: ParameterCount::Zero,
        });

        self
    }

    /// Adds a flag which only has a long version
    pub fn add_long_flag(mut self, name: String, long: String) -> Self {
        self.args.push(Arg {
            short: None,
            long: Some(long),
            name,
            kind: ArgType::Option,
            parameter_count: ParameterCount::Zero,
        });

        self
    }

    /// Adds a subcommand
    pub fn add_subcommand(mut self, name: String, parameter_count: ParameterCount) -> Self {
        self.args.push(Arg {
            short: None,
            long: None,
            name,
            kind: ArgType::Subcommand,
            parameter_count,
        });

        self
    }

    /// Builds ```Config```
    pub fn build(self) -> Config {
        Config {
            args: self.args,
        }
    }
}

/// Represents Configuration for CLI Arguments
pub struct Config {
    pub args: Vec<Arg>,
}