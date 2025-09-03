use crate::routes::{LocationPair, LocationPairList, Route};

pub fn run() {
    // let input_data = common::read_input!("day9");
    let input_data = r#"
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141
    "#;

    let loc_pairs = LocationPairList::from_input_data(input_data);
    println!("{loc_pairs}");
    let route = Route::from_location_pair_list(loc_pairs);
    println!("{route}");
}
