mod look_say;
mod part1;
mod part2;
use anyhow::Result;

fn main() -> Result<()> {
    part1::run()?;
    part2::run()?;
    Ok(())
}
