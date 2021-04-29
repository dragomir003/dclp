mod error;

use std::env;
use std::iter::{Iterator, Peekable};

use crate::arg::{Arg, ArgType, Args, ParameterCount};
use crate::config::Config;
pub use error::ParseError;

/// Parses CLI Arguments with the help of a Config
/// # Returns
/// If everything is ok returns ```Args``` else an Error
pub fn parse(arg_config: Config) -> Result<Args, ParseError> {
    let mut args = env::args();
    let program_name = args.next().ok_or(ParseError::NoProgramName)?;
    parse_inputs(arg_config, args.peekable(), program_name)
}

fn parse_inputs(
    config: Config,
    mut args: Peekable<impl Iterator<Item = String>>,
    program_name: String,
) -> Result<Args, ParseError> {
    let subcommands: Vec<_> = config
        .args
        .iter()
        .filter(|arg| arg.kind == ArgType::Subcommand)
        .collect();
    let options: Vec<_> = config
        .args
        .iter()
        .filter(|arg| arg.kind == ArgType::Option)
        .collect();

    let mut result = Args::new();

    for arg in subcommands.iter().chain(options.iter()) {
        result.insert(arg.name.clone(), None);
    }
    result.insert(program_name.clone(), Some(vec![]));

    while let Some(arg) = args.next() {
        if arg.starts_with('-') {
            let option = option_exists(&arg, &options).ok_or(ParseError::InvalidOption(arg))?;

            assign_parameters(
                &mut args,
                option.parameter_count,
                result.get_mut(&option.name).unwrap().get_or_insert(vec![]),
                &subcommands,
                &options,
                option,
            )?;
        } else {
            if let Some(subcommand) = subcommands.iter().find(|a| a.name == arg) {
                assign_parameters(
                    &mut args,
                    subcommand.parameter_count,
                    result
                        .get_mut(&subcommand.name)
                        .unwrap()
                        .get_or_insert(vec![]),
                    &subcommands,
                    &options,
                    subcommand,
                )?;
            } else {
                let val = result.get_mut(&program_name).unwrap().as_mut().unwrap();
                val.push(arg);
            }
        }
    }

    Ok(result)
}

fn is_option_or_subcommand<'a>(
    s: &str,
    options: &Vec<&'a Arg>,
    subcommands: &Vec<&'a Arg>,
) -> Option<&'a Arg> {
    if s.starts_with('-') {
        option_exists(s, options)
    } else {
        subcommand_exists(s, subcommands)
    }
}

fn option_exists<'a>(opt: &str, options: &Vec<&'a Arg>) -> Option<&'a Arg> {
    if opt.starts_with("--") {
        let long = &opt[2..];
        options
            .iter()
            .filter(|opt| opt.long.is_some())
            .find(|opt| opt.long.as_ref().unwrap() == long)
            .map(|opt| *opt)
    } else {
        let c = opt.chars().skip(1).next()?;
        options
            .iter()
            .filter(|opt| opt.short.is_some())
            .find(|opt| opt.short.unwrap() == c)
            .map(|opt| *opt)
    }
}

fn subcommand_exists<'a>(sub: &str, subcommands: &Vec<&'a Arg>) -> Option<&'a Arg> {
    subcommands
        .iter()
        .find(|com| com.name == sub)
        .map(|sub| *sub)
}

fn assign_parameters(
    args: &mut Peekable<impl Iterator<Item = String>>,
    parameter_count: ParameterCount,
    params: &mut Vec<String>,
    subcommands: &Vec<&Arg>,
    options: &Vec<&Arg>,
    arg: &Arg,
) -> Result<(), ParseError> {
    match parameter_count {
        ParameterCount::Zero => {}
        ParameterCount::Exact(n) => {
            for i in 1..=n {
                let param = args
                    .next()
                    .ok_or(ParseError::InvalidNumberOfParameters(format!(
                        "There are only {} parameters to supply {} with instead of {}",
                        i, arg, n,
                    )))?;
                if (param.starts_with('-') && option_exists(&param, options).is_some())
                    || subcommand_exists(&param, subcommands).is_some()
                {
                    return Err(ParseError::InvalidNumberOfParameters(format!(
                        "There are only {} parameters to supply {} with instead of {}",
                        i, arg, n,
                    )));
                }
                params.push(param);
            }
        }
        ParameterCount::More(n) => {
            for param_count in 0.. {
                if let Some(param) = args.peek() {
                    let is_opt_or_sub =
                        is_option_or_subcommand(param, options, subcommands).is_some();

                    if is_opt_or_sub && param_count > n {
                        break;
                    } else if is_opt_or_sub && param_count <= n {
                        return Err(ParseError::InvalidNumberOfParameters(format!(
                            "{} expected at least {} parameters but got {}",
                            arg,
                            n + 1,
                            param_count
                        )));
                    }
                    let param = args.next().unwrap();
                    params.push(param);
                } else {
                    if param_count <= n {
                        return Err(ParseError::InvalidNumberOfParameters(format!(
                            "{} expected at least {} parameters but got {}",
                            arg,
                            n + 1,
                            param_count
                        )));
                    }
                    break;
                }
            }
        }
        ParameterCount::Less(n) => {
            for _ in 0..n-1 {
                if let Some(param) = args.peek() {
                    if is_option_or_subcommand(param, options, subcommands).is_some() {
                        break;
                    }
                    let param = args.next().unwrap();
                    params.push(param)
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ConfigBuilder;

    #[test]
    fn short_option_param_count_zero() {
        let config = ConfigBuilder::default()
            .add_short_flag("test1".into(), 't')
            .add_short_flag("test2".into(), 'u')
            .build();
        let args = vec!["-t".into(), "-u".into()].into_iter();

        let result = parse_inputs(
            config,
            args.into_iter().peekable(),
            "short_option_param_count_zero".into(),
        );

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(&Some(vec![]), result.get("test1").unwrap());
        assert_eq!(&Some(vec![]), result.get("test2").unwrap());
    }

    #[test]
    fn short_option_param_count_exact() {
        let config = ConfigBuilder::default()
            .add_short_option("test1".into(), 't', ParameterCount::Exact(3))
            .add_short_option("test2".into(), 'u', ParameterCount::Exact(1))
            .add_short_option("test3".into(), 'v', ParameterCount::Exact(4))
            .build();
        let args = vec![
            "-v",
            "a",
            "b",
            "c",
            "h",
            "-t",
            "d",
            "test",
            "e",
            "-u",
            "this is the last one",
        ]
        .into_iter()
        .map(|s| String::from(s));

        let result = parse_inputs(
            config,
            args.into_iter().peekable(),
            "short_option_param_count_exact".into(),
        );

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(
            &Some(vec!["d".into(), "test".into(), "e".into()]),
            result.get("test1").unwrap()
        );
        assert_eq!(
            &Some(vec!["this is the last one".into()]),
            result.get("test2").unwrap()
        );
        assert_eq!(
            &Some(vec!["a".into(), "b".into(), "c".into(), "h".into()]),
            result.get("test3").unwrap()
        );
    }

    #[test]
    fn short_option_param_count_zero_and_exact() {
        let config = ConfigBuilder::default()
            .add_short_option("test1".into(), 't', ParameterCount::Zero)
            .add_short_option("test2".into(), 'u', ParameterCount::Exact(3))
            .add_short_option("test3".into(), 'v', ParameterCount::Exact(2))
            .add_short_option("test4".into(), 'w', ParameterCount::Zero)
            .build();
        let args = vec![
            "-w",
            "-v",
            "a",
            "b",
            "-t",
            "-u",
            "this is the last one",
            "not yet there",
            "now we're done",
        ]
        .into_iter()
        .map(|s| String::from(s));

        let result = parse_inputs(
            config,
            args.into_iter().peekable(),
            "short_option_param_count_zero_and_exact".into(),
        );

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(&Some(vec![]), result.get("test1").unwrap());
        assert_eq!(
            &Some(vec![
                "this is the last one".into(),
                "not yet there".into(),
                "now we're done".into()
            ]),
            result.get("test2").unwrap()
        );
        assert_eq!(
            &Some(vec!["a".into(), "b".into()]),
            result.get("test3").unwrap()
        );
        assert_eq!(&Some(vec![]), result.get("test4").unwrap());
    }

    #[test]
    fn subcommand_param_count_zero() {
        let config = ConfigBuilder::default()
            .add_subcommand("test1".into(), ParameterCount::Zero)
            .add_subcommand("test2".into(), ParameterCount::Zero)
            .build();

        let args = vec!["hello", "test1", "hello again", "test2"]
            .into_iter()
            .map(|s| String::from(s));

        let result = parse_inputs(
            config,
            args.into_iter().peekable(),
            "subcommand_param_count_zero".into(),
        );

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(
            &Some(vec!["hello".into(), "hello again".into()]),
            result.get("subcommand_param_count_zero").unwrap()
        );
        assert_eq!(&Some(vec![]), result.get("test1").unwrap());
        assert_eq!(&Some(vec![]), result.get("test2").unwrap());
    }

    #[test]
    fn subcommand_param_count_zero_and_exact() {
        let config = ConfigBuilder::default()
            .add_subcommand("test1".into(), ParameterCount::Exact(2))
            .add_subcommand("test2".into(), ParameterCount::Zero)
            .build();

        let args = vec!["hello", "test1", "hello again", "not yet test2", "test2"]
            .into_iter()
            .map(|s| String::from(s));

        let result = parse_inputs(
            config,
            args.into_iter().peekable(),
            "subcommand_param_count_zero".into(),
        );

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(
            &Some(vec!["hello".into()]),
            result.get("subcommand_param_count_zero").unwrap()
        );
        assert_eq!(
            &Some(vec!["hello again".into(), "not yet test2".into()]),
            result.get("test1").unwrap()
        );
        assert_eq!(&Some(vec![]), result.get("test2").unwrap());
    }

    #[test]
    fn long_option() {
        let config = ConfigBuilder::default()
            .add_long_option("test1".into(), "test1".into(), ParameterCount::Exact(2))
            .add_long_flag("test2".into(), "test2".into())
            .build();

        let args = vec![
            "hello",
            "--test1",
            "hello again",
            "not yet test2",
            "--test2",
        ]
        .into_iter()
        .map(|s| String::from(s));

        let result = parse_inputs(config, args.into_iter().peekable(), "long_option".into())
            .unwrap_or_else(|e| {
                println!("{}", e);
                panic!("");
            });

        assert_eq!(
            &Some(vec!["hello".into()]),
            result.get("long_option").unwrap()
        );
        assert_eq!(
            &Some(vec!["hello again".into(), "not yet test2".into()]),
            result.get("test1").unwrap()
        );
        assert_eq!(&Some(vec![]), result.get("test2").unwrap());
    }

    #[test]
    fn parameter_count_more() {
        let config = ConfigBuilder::default()
            .add_long_option("test1".into(), "test1".into(), ParameterCount::More(2))
            .add_short_option("test2".into(), 't', ParameterCount::Exact(1))
            .add_subcommand("sub".into(), ParameterCount::Zero)
            .build();

        let args = vec![
            "hello",
            "--test1",
            "hello again",
            "not yet test2",
            "third arg",
            "fourth arg",
            "sub",
            "-t",
            "arg for test2",
        ]
        .into_iter()
        .map(|s| String::from(s));

        let result = parse_inputs(
            config,
            args.into_iter().peekable(),
            "parameter_count_more".into(),
        )
        .unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(
            &Some(vec!["hello".into()]),
            result.get("parameter_count_more").unwrap()
        );
        assert_eq!(
            &Some(vec![
                "hello again".into(),
                "not yet test2".into(),
                "third arg".into(),
                "fourth arg".into()
            ]),
            result.get("test1").unwrap()
        );
        assert_eq!(
            &Some(vec!["arg for test2".into()]),
            result.get("test2").unwrap()
        );
        assert_eq!(&Some(vec![]), result.get("sub").unwrap())
    }

    #[test]
    fn parameter_count_less() {
        let config = ConfigBuilder::default()
            .add_long_option("test1".into(), "test1".into(), ParameterCount::More(1))
            .add_short_option("test2".into(), 't', ParameterCount::Exact(2))
            .add_subcommand("sub".into(), ParameterCount::Less(3))
            .build();

        let args = vec![
            "0arg", "-t", "1arg", "2arg", "--test1", "3arg", "4arg", "sub", "5arg", "6arg", "7arg",
        ]
        .into_iter()
        .map(|s| String::from(s));

        let result = parse_inputs(
            config,
            args.into_iter().peekable(),
            "parameter_count_less".into(),
        )
        .unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(
            &Some(vec!["0arg".into(), "7arg".into()]),
            result.get("parameter_count_less").unwrap()
        );
        assert_eq!(
            &Some(vec!["3arg".into(), "4arg".into()]),
            result.get("test1").unwrap()
        );
        assert_eq!(
            &Some(vec!["1arg".into(), "2arg".into()]),
            result.get("test2").unwrap()
        );
        assert_eq!(
            &Some(vec!["5arg".into(), "6arg".into()]),
            result.get("sub").unwrap()
        )
    }
}
