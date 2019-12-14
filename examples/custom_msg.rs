use context_attribute::context;
use failure::{Error, ResultExt};

use std::fs;

trait ShowErr {
    fn show_err(&self);
}

impl<T> ShowErr for Result<T, failure::Error> {
    fn show_err(&self) {
        if let Err(e) = self {
            println!("{}", pretty_error(e));
        }
    }
}

fn pretty_error(err: &failure::Error) -> String {
    let mut pretty = err.to_string();
    let mut prev = err.cause();
    while let Some(next) = prev.cause() {
        pretty.push_str(": ");
        pretty.push_str(&next.to_string());
        prev = next;
    }
    pretty
}

fn read_file_1() -> Result<String, Error> {
    let res = fs::read_to_string("address.txt")
        .context("error reading address.txt from disk")?
        .trim()
        .to_string();
    Ok(res)
}

/// Read address.txt from disk
#[context]
fn read_file_2() -> Result<String, Error> {
    Ok(fs::read_to_string("address.txt")?.trim().to_string())
}

#[context(fn)]
fn read_file_3() -> Result<String, Error> {
    Ok(fs::read_to_string("address.txt")?.trim().to_string())
}

/// Read address.txt from disk
#[context(doc)]
fn read_file_4() -> Result<String, Error> {
    Ok(fs::read_to_string("address.txt")?.trim().to_string())
}

#[context(msg:"xxxxxx")]
fn read_file_5() -> Result<String, Error> {
    Ok(fs::read_to_string("address.txt")?.trim().to_string())
}

fn main() -> Result<(), Error> {
    read_file_1().show_err();
    read_file_2().show_err();
    read_file_3().show_err();
    read_file_4().show_err();
    read_file_5().show_err();

    Ok(())
}
