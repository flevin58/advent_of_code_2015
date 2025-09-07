use crate::password::Password;

pub fn run() -> Result<(), String> {
    let input = common::read_input(11)?;
    let mut pw = Password::from_str(&input);
    println!("Next valid password: {}", pw.next_valid_password());
    println!("Next valid password: {}", pw.next_valid_password());
    Ok(())
}
