use crate::round_table::RoundTable;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(13)?;
    let rt = RoundTable::from_input(&input.to_string())?;
    let max_points = rt.get_max_happiness()?;
    println!("The best disposition has {max_points} points");
    Ok(())
}
