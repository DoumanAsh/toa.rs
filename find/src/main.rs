#[macro_use]
extern crate common;

extern crate regex;
extern crate walkdir;

use std::path;
use std::process::exit;

mod cli;
mod find;

fn run() -> Result<i32, String> {
    let args = match cli::Args::new() {
        Ok(args) => args,
        Err(error) => return Err(error)
    };

    Ok(find::Find::from_parser(args).run())
}

fn main() {
    let code: i32 = match run() {
        Ok(res) => res,
        Err(error) => {
            eprintln!("{}", error);
            1
        }
    };

    exit(code);
}

