use crate::grid::Grid;

pub fn run() {
    let input_data = common::read_input!("day6");
    let mut grid = Grid::new(false);
    for line in input_data.lines() {
        grid.apply_action(line);
    }
    let count = grid.count_lights_on();
    println!("Number of lights on: {count}");
}
