#[macro_export]
macro_rules! read_input {
    ($day: expr) => {
        std::fs::read_to_string(concat!("src/bin/", $day, "/input.txt")).unwrap()
    };
}
