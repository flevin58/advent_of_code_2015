pub fn after_iters(start: &str, n: i32) -> String {
    let mut start = String::from(start);
    for _ in 0..n {
        let end = read_aloud(&start);
        // println!("{start} -> {end}");
        start = end;
    }
    start
}

// Helper function
fn read_aloud(s: &String) -> String {
    let mut current: Option<char> = None;
    let mut count = 0;
    let mut vu8 = Vec::<u8>::new();
    for ch in s.chars() {
        match current {
            Some(cur_ch) => {
                if cur_ch == ch {
                    count += 1;
                } else {
                    vu8.push(count);
                    vu8.push((cur_ch as u8) - 0x30);
                    count = 1;
                    current = Some(ch);
                }
            }
            None => {
                count = 1;
                current = Some(ch);
            }
        }
    }
    vu8.push(count);
    vu8.push((current.unwrap() as u8) - 0x30);
    vu8.iter().map(|&d| (d + b'0') as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_after_iters() {
        let result = after_iters("1", 5);
        assert_eq!(result, String::from("312211"));
    }
}
