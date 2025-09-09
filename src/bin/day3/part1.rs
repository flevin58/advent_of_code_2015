use crate::santa;
use common::error::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(3)?;
    let total_houses = santa::visited_houses(&input)?;
    println!("Visited houses: {}", total_houses);
    Ok(())
}
