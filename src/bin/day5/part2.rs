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

pub fn run() {
    let input_data = common::read_input!("day5");
    let count = input_data.lines().filter(|line| is_nice(line)).count();
    println!("Number of 'nice' strings: {count}");
}

#[cfg(test)]
mod tests {
    type TestCase = (&'static str, bool);

    const TEST_CASES: [TestCase; 4] = [
        ("qjhvhtzxzqqjkmpb", true),
        ("xxyxx", true),
        ("uurcxstgmygtbstg", false),
        ("ieodomkazucvgmuy", false),
    ];

    #[test]
    fn test_is_nice() {
        for tc in TEST_CASES {
            let result = super::is_nice(tc.0);
            assert_eq!(result, tc.1);
        }
    }
}
