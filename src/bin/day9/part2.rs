use crate::routes::Routes;
use common::error::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(9)?;
    let routes = Routes::from_input_data(&input)?;
    println!(
        "longest route distance: {}",
        routes.get_longest_route_distance()
    );
    Ok(())
}
