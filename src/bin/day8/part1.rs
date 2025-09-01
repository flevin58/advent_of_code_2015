use crate::delta::part1_delta_chars;

pub fn run() {
    let input_data: String = common::read_input!("day8");
    let delta = input_data
        .lines()
        .fold(0, |acc: usize, line: &str| acc + part1_delta_chars(line));
    println!("Difference: {}", delta);
}
