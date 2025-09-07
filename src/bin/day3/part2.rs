use crate::santa;

pub fn run() -> Result<(), String> {
    let input = common::read_input(3)?;
    let total_houses = santa::visited_houses_with_robot(&input);
    println!("Visited houses: {}", total_houses);
    Ok(())
}
