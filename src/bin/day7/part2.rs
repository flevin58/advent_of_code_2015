use crate::circuit::Wires;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = common::read_input(7)?;
    let mut wires = Wires::from_input_data(&input)?;
    wires.set_wire_signal("b", 3176);
    wires.compute()?;
    println!(
        "Part2: value on wire 'a': {}",
        wires.get_wire_signal("a").unwrap()
    );
    Ok(())
}
