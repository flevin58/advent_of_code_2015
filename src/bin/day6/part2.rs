use crate::grid::Grid;

pub fn run() {
    let input_data = common::read_input!("day6");
    let mut grid = Grid::new(true);
    for line in input_data.lines() {
        grid.apply_action(line);
    }
    let total_brightness = grid.sum_brightness();
    println!("Total brightness: {total_brightness}");
}
