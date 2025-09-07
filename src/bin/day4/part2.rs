use crate::password;

pub fn run() -> Result<(), String> {
    let input = common::read_input(4)?;
    let result = password::lowest_number_with_n_leading_zeros(&input, 6);
    println!("Valid passwords: {}", result);
    Ok(())
}
