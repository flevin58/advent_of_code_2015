use crate::routes::Routes;

pub fn run() -> Result<(), String> {
    let input = common::read_input(9)?;
    let routes = Routes::from_input_data(&input);
    println!(
        "shortest route distance: {}",
        routes.get_shortest_route_distance()
    );
    Ok(())
}
