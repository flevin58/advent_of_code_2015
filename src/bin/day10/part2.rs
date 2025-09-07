use crate::look_say::after_iters;

pub fn run() -> Result<(), String> {
    let input = common::read_input(10)?;
    let result = after_iters(input.as_str(), 50);
    println!("Result after 50 times has length: {}", result.len());
    Ok(())
}
