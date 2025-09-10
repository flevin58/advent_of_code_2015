use crate::reindeer::ReindeerList;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(14)?;
    let mut rdv = ReindeerList::from_str(&input)?;
    let secs = 2503;
    let points = rdv.max_winning_points_after(secs);
    println!("The max points after {secs}s is: {points}");
    Ok(())
}
