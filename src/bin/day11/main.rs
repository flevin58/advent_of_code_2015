mod part1_2;
mod password;
use anyhow::Result;

fn main() -> Result<()> {
    part1_2::run()?;
    Ok(())
}
