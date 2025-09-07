use crate::grid::Grid;

pub fn run() -> Result<(), String> {
    let input = common::read_input(6)?;
    let mut grid = Grid::new(false);
    for line in input.lines() {
        grid.apply_action(line);
    }
    let count = grid.count_lights_on();
    println!("Number of lights on: {count}");
    Ok(())
}
