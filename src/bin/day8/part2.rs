use crate::delta::part2_delta_chars;

pub fn run() -> Result<(), String> {
    let input = common::read_input(8)?;
    let delta = input
        .lines()
        .fold(0, |acc: usize, line: &str| acc + part2_delta_chars(line));
    println!("Difference: {}", delta);
    Ok(())
}
