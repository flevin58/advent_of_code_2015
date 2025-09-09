use crate::delta::part1_delta_chars;
use common::error::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(8)?;
    let delta = input.lines().try_fold(0, |acc: usize, line: &str| {
        part1_delta_chars(line).map(|v| acc + v)
    })?;
    println!("Difference: {}", delta);
    Ok(())
}
