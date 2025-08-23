use crate::santa;

pub fn run() {
    let input_data = common::read_input!("day3");
    let total_houses = santa::visited_houses(&input_data);
    println!("Visited houses: {}", total_houses);
}
