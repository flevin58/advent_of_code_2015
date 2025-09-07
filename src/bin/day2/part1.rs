use crate::prism::Prism;

fn amount_of_wrapping_paper(single_line: &str) -> u32 {
    let prism = Prism::from_str(single_line);
    prism.total_wrapping_paper()
}

pub fn run() -> Result<(), String> {
    let input = common::read_input(2)?;
    let total_sum: u32 = input
        .lines()
        .map(|line| amount_of_wrapping_paper(line))
        .sum();
    println!("Total wrapping paper = {}", total_sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("2x3x4", 58; "measure 2x3x4")]
    #[test_case("1x1x10", 43; "measure 1x1x10")]
    fn amount_of_wrapping_paper(measures: &str, expected: u32) {
        let result = super::amount_of_wrapping_paper(measures);
        assert_eq!(result, expected);
    }
}
