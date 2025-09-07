pub fn read_input(day: u8) -> Result<String, String> {
    let path = format!("src/bin/day{day}/input.txt");
    let input = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    Ok(input)
}
