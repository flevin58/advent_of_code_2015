use crate::routes::Routes;

pub fn run() {
    let input_data = common::read_input!("day9");
    let routes = Routes::from_input_data(&input_data);
    println!(
        "longest route distance: {}",
        routes.get_longest_route_distance()
    );
}
