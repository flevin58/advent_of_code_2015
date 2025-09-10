mod part1;
mod part2;
mod reindeer;
use anyhow::Result;

fn main() -> Result<()> {
    part1::run()?;
    part2::run()?;
    Ok(())
}
