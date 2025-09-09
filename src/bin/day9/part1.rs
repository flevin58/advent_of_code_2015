use crate::routes::Routes;
use common::error::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(9)?;
    let routes = Routes::from_input_data(&input)?;
    println!(
        "shortest route distance: {}",
        routes.get_shortest_route_distance()
    );
    Ok(())
}
