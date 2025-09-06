use crate::password::Password;

pub fn run() {
    let input_data = common::read_input!("day11");
    let mut pw = Password::from_str(&input_data);
    println!("Next valid password: {}", pw.next_valid_password());
    println!("Next valid password: {}", pw.next_valid_password());
}
