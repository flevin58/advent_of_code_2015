mod part1;
mod part2;
mod password;
fn main() -> Result<(), String> {
    part1::run()?;
    part2::run()?;
    Ok(())
}
