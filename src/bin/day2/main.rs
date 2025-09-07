mod part1;
mod part2;
mod prism;
fn main() -> Result<(), String> {
    part1::run()?;
    part2::run()?;
    Ok(())
}
