mod part1;
mod part2;
mod prism;

use common::error::Result;
fn main() -> Result<()> {
    part1::run()?;
    part2::run()?;
    Ok(())
}
