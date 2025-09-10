use crate::password;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(4)?;
    let result = password::lowest_number_with_n_leading_zeros(&input, 5);
    println!("Valid passwords: {}", result);
    Ok(())
}
