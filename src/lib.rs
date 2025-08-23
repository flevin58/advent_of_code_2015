#[macro_export]
macro_rules! read_input {
    ($day: expr) => {
        match std::fs::read_to_string(concat!("src/bin/", $day, "/input.txt")) {
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                return;
            }
            Ok(data) => data,
        };
    };
}
