use crate::password::Password;
use common::error::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(11)?;
    let mut pw = Password::from_str(&input);
    println!("Next valid password: {}", pw.next_valid_password());
    println!("Next valid password: {}", pw.next_valid_password());
    Ok(())
}
