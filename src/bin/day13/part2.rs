use crate::round_table::RoundTable;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(13)?;
    let mut rt = RoundTable::from_input(&input.to_string())?;
    let max_points = rt.get_max_happiness_with_me()?;
    println!("The best disposition including myself has {max_points} points");
    Ok(())
}
