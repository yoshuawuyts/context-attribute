use context_attribute::context;
use failure::{ensure, ResultExt};

/// Square a number if it's less than 10.
#[context]
fn square(num: usize) -> Result<usize, failure::Error> {
    ensure!(num < 10, "Number was too large");
    Ok(num * num)
}

fn main() -> Result<(), failure::Error> {
    let args = std::env::args();
    ensure!(args.len() == 2, "usage: square <num>");
    let input = args.skip(1).next().unwrap().parse()?;

    println!("result is {}", square(input)?);

    Ok(())
}
