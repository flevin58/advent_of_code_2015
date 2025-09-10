use crate::santa;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(3)?;
    let total_houses = santa::visited_houses_with_robot(&input)?;
    println!("Visited houses: {}", total_houses);
    Ok(())
}
