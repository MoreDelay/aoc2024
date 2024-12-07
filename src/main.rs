use anyhow::Result;

mod util;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() -> Result<()> {
    day01::run()?;
    day02::run()?;
    day03::run()?;
    day04::run()?;
    Ok(())
}
