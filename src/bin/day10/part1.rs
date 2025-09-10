use crate::look_say::after_iters;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(10)?;
    let result = after_iters(input.as_str(), 40);
    println!("Result after 40 times has length: {}", result.len());
    Ok(())
}
