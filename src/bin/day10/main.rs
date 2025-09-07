mod look_say;
mod part1;
mod part2;

fn main() -> Result<(), String> {
    part1::run()?;
    part2::run()?;
    Ok(())
}
