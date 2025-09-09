use common::error::AocError;

use crate::floor;

pub fn run() -> Result<(), AocError> {
    let input = common::read_input(1)?;
    let index = floor::index_of_basement(&input)?;
    println!("Index of basement = {index}");
    Ok(())
}
