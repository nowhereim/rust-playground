use crate::domain;

pub fn get_hello_message() -> &'static str {
    "Hello, world!"
}

pub fn get_hi_message() -> &'static str {
    "Hi there!"
}

pub fn get_calculation_result() -> String {
    let sum = domain::calculate_sum();
    format!("계산 결과: {}", sum)
}
