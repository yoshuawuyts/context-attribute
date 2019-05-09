use context_attribute::context;
use failure::{Error, ResultExt};

use std::fs;
use std::net::SocketAddr;

/// Read address.txt from disk
fn read_file_1() -> Result<String, Error> {
    let res = std::fs::read_to_string("address.txt")
        .context("error reading address.txt from disk")?
        .trim()
        .to_string();
    Ok(res)
}

/// Read address.txt from disk
#[context]
fn read_file_2() -> Result<String, Error> {
    Ok(std::fs::read_to_string("address.txt")?.trim().to_string())
}

fn main() -> Result<(), Error> {
    read_file_1()?;
    read_file_2()?;
    Ok(())
}
