use super::Args;

/// Checks if argument with the name appears in args
pub fn appeared(name: &str, args: &Args) -> bool {
    args.get(name).is_some()
}

/// Gets parameters to an argument with the name
/// # Returns
/// ```Some(params)``` if argument appeared, else ```None```
pub fn parameters<'a>(name: &str, args: &'a Args) -> Option<&'a Vec<String>> {
    args.get(name)?.as_ref()
}