mod part1_2;
mod password;
use common::error::Result;

fn main() -> Result<()> {
    part1_2::run()?;
    Ok(())
}
