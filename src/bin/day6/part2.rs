use crate::grid::Grid;

pub fn run() -> Result<(), String> {
    let input = common::read_input(6)?;
    let mut grid = Grid::new(true);
    for line in input.lines() {
        grid.apply_action(line);
    }
    let total_brightness = grid.sum_brightness();
    println!("Total brightness: {total_brightness}");
    Ok(())
}
