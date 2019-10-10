use context_attribute::context;
use failure::ensure;

struct Counter(usize);

impl Counter {
    /// Counts down until the target number
    #[context]
    fn count(&mut self, target: usize) -> Result<(), failure::Error> {
        ensure!(self.0 >= target, "Target is greater than current count");

        while self.0 > target {
            println!("{}", self.0);
            self.0 -= 1;
        }

        Ok(())
    }

    /// Prints the numbers down to a target number without changing the current count
    #[context]
    fn print(&self, target: usize) -> Result<(), failure::Error> {
        ensure!(self.0 >= target, "Target is greater than current count");

        while self.0 > target {
            println!("{}", self.0);
        }

        Ok(())
    }
}

fn main() -> Result<(), failure::Error> {
    ensure!(std::env::args().len() == 3, "usage: counter <num> <target>");
    let input = std::env::args().skip(1).next().unwrap().parse()?;
    let target = std::env::args().skip(2).next().unwrap().parse()?;

    let mut counter = Counter(input);

    counter.print(target)?;
    counter.count(target)?;

    Ok(())
}
