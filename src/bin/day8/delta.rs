enum State {
    Idle,
    InString,
    EscapeChar,
    SkipFirstHexDigit,
    SkipSecondHexDigit,
}

pub fn part1_delta_chars(line: &str) -> usize {
    let mut delta = 2_usize; // We take into account the starting and ending double quotes, not in mem!
    let mut state = State::Idle;

    for ch in line.trim().chars() {
        match state {
            State::Idle => {
                assert_eq!(ch, '"', "The line must start with a double quote");
                state = State::InString;
            }
            State::InString => {
                if ch == '"' {
                    return delta;
                }

                if ch == '\\' {
                    state = State::EscapeChar;
                }
            }
            State::EscapeChar => match ch {
                '\\' | '"' => {
                    delta += 1;
                    state = State::InString;
                }
                'x' => {
                    delta += 3;
                    state = State::SkipFirstHexDigit;
                }
                _ => {
                    panic!("Illegal escape character in line");
                }
            },
            State::SkipFirstHexDigit => {
                assert!(ch.is_ascii_hexdigit());
                state = State::SkipSecondHexDigit;
            }
            State::SkipSecondHexDigit => {
                assert!(ch.is_ascii_hexdigit());
                state = State::InString;
            }
        }
    }
    unreachable!();
}

pub fn part2_delta_chars(line: &str) -> usize {
    let mut delta = 2_usize; // We take into account the starting and ending double quotes, not in mem!
    let mut state = State::Idle;

    for ch in line.trim().chars() {
        match state {
            State::Idle => {
                assert_eq!(ch, '"', "The line must start with a double quote");
                delta += 1;
                state = State::InString;
            }
            State::InString => {
                if ch == '"' || ch == '\\' {
                    delta += 1;
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
    delta
}

#[cfg(test)]

mod tests {
    use crate::delta::{part1_delta_chars, part2_delta_chars};
    use test_case::test_case;

    #[test_case(r#""""#, 2 ; "empty line") ]
    #[test_case(r#""abc""#, 2 ; "with quoted string") ]
    #[test_case(r#""aaa\"aaa""#, 3 ; "with escaped double quote") ]
    #[test_case(r#""\x27""#, 5 ; "with escaped hex char") ]
    fn part1_delta(line: &str, expected: usize) {
        let delta = part1_delta_chars(line);
        assert_eq!(delta, expected);
    }

    #[test_case(r#""""#, 4 ; "empty line") ]
    #[test_case(r#""abc""#, 4 ; "with quoted string") ]
    #[test_case(r#""aaa\"aaa""#, 6 ; "with escaped double quote") ]
    #[test_case(r#""\x27""#, 5 ; "with escaped hex char") ]
    fn part2_delta(line: &str, expected: usize) {
        let delta = part2_delta_chars(line);
        assert_eq!(delta, expected);
    }
}
