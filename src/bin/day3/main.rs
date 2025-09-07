mod part1;
mod part2;
mod santa;
fn main() -> Result<(), String> {
    part1::run()?;
    part2::run()?;
    Ok(())
}
