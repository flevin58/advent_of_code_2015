use crate::circuit::Wires;

pub fn run() {
    let input_data = common::read_input!("day7");
    let mut wires = Wires::from_input_data(&input_data);
    wires.compute();
    println!(
        "Part1: value on wire 'a': {:?}",
        wires.get_wire("a").unwrap()
    );
}
