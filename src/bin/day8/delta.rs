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

    #[test]
    fn test_part1_delta_chars() {
        let delta = part1_delta_chars(r#""""#);
        assert_eq!(delta, 2);

        let delta = part1_delta_chars(r#""abc""#);
        assert_eq!(delta, 2);

        let delta = part1_delta_chars(r#""aaa\"aaa""#);
        assert_eq!(delta, 3);

        let delta = part1_delta_chars(r#""\x27""#);
        assert_eq!(delta, 5);
    }

    #[test]
    fn test_part2_delta_chars() {
        let delta = part2_delta_chars(r#""""#);
        assert_eq!(delta, 4);

        let delta = part2_delta_chars(r#""abc""#);
        assert_eq!(delta, 4);

        let delta = part2_delta_chars(r#""aaa\"aaa""#);
        assert_eq!(delta, 6);

        let delta = part2_delta_chars(r#""\x27""#);
        assert_eq!(delta, 5);
    }
}
