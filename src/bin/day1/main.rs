mod floor;
mod part1;
mod part2;

use common::error::AocError;
fn main() -> Result<(), AocError> {
    part1::run()?;
    part2::run()?;
    Ok(())
}
