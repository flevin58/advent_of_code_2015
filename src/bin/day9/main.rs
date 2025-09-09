mod part1;
mod part2;
mod routes;
use common::error::Result;
fn main() -> Result<()> {
    part1::run()?;
    part2::run()?;
    Ok(())
}
