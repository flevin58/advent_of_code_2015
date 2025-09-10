use crate::reindeer::ReindeerList;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(14)?;
    let rdv = ReindeerList::from_str(&input)?;
    let secs = 2503;
    let wdist = rdv.winning_distance_after(secs);
    println!("The winning distance after {secs}s is: {wdist}");
    Ok(())
}
