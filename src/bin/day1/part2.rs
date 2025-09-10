use crate::floor;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(1)?;
    let index = floor::index_of_basement(&input)?;
    println!("Index of basement = {index}");
    Ok(())
}
