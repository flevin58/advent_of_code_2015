fn is_nice(s: &str) -> bool {
    // Rule 1: it contains a pair of any two letters that appears at least twice in the string without overlapping
    let has_pair = (0..s.len() - 1).any(|i| {
        let pair = &s[i..i + 2];
        s[i + 2..].contains(pair)
    });
    if !has_pair {
        return false;
    }

    // Rule 2: it contains at least one letter which repeats with exactly one letter between them
    let has_repeat_with_gap = s.chars().zip(s.chars().skip(2)).any(|(a, b)| a == b);
    if !has_repeat_with_gap {
        return false;
    }

    // If it passes all the rules, it's nice
    true
}

pub fn run() -> Result<(), String> {
    let input = common::read_input(5)?;
    let count = input.lines().filter(|line| is_nice(line)).count();
    println!("Number of 'nice' strings: {count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("qjhvhtzxzqqjkmpb", true; "t2.1")]
    #[test_case("xxyxx", true; "t2.2")]
    #[test_case("uurcxstgmygtbstg", false; "t2.3")]
    #[test_case("ieodomkazucvgmuy", false; "t2.4")]
    fn test_is_nice(input: &str, expected: bool) {
        let result = super::is_nice(input);
        assert_eq!(result, expected);
    }
}
