use anyhow::Result;

pub fn read_input(day: u8) -> Result<String> {
    let path = format!("src/bin/day{day}/input.txt");
    let input = std::fs::read_to_string(path)?;
    Ok(input)
}
