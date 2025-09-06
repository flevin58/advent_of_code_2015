use crate::look_say::after_iters;

pub fn run() {
    let input_data = common::read_input!("day10");
    let result = after_iters(input_data.as_str(), 50);
    println!("Result after 50 times has length: {}", result.len());
}
