use crate::floor;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(1)?;
    let floor = floor::floor_number_from_string(&input)?;
    println!("Floor = {floor}");
    Ok(())
}
