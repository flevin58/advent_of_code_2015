use crate::password;

pub fn run() {
    let input_data = common::read_input!("day4");
    let result = password::lowest_number_with_n_leading_zeros(&input_data, 6);
    println!("Valid passwords: {}", result);
}
