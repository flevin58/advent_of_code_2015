use crate::floor;

pub fn run() -> Result<(), String> {
    let input = common::read_input(1)?;
    let floor = floor::floor_number_from_string(&input);
    println!("Floor = {floor}");
    Ok(())
}
