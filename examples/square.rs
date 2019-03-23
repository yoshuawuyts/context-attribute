use context_attribute::context;
use failure::ensure;

fn main() -> Result<(), failure::Error> {
    let _ = square(2)?;
    let _ = square(5)?;
    let _ = square(11)?;
    Ok(())
}

/// Square a number if it's less than 10.
#[context]
fn square(num: usize) -> Result<String, failure::Error> {
    ensure!(num < 10, "Number was larger than 10");
    Ok(num * num)
}
