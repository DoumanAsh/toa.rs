extern crate clap;

use ::std::fmt::Display;
use ::std::str::FromStr;
pub use self::clap::{App, Arg, ArgMatches};

#[inline(always)]
///Shortcut to create CLI argument
pub fn arg(name: &str) -> Arg {
    Arg::with_name(name)
}

#[inline(always)]
///Shortcut to create CLI option/flag
pub fn flag(name: &str) -> Arg {
    arg(name).long(name)
}

#[inline(always)]
///Shortcut to parse integer
pub fn parse_int<T: FromStr>(name: &str) -> Result<T, String>
    where <T as FromStr>::Err: Display
{
    name.parse::<T>().map_err(|error| format!("Invalid number '{}' is supplied. {}", name, error))
}
