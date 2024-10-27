use crate::infrastructure;
use diesel::mysql::MysqlConnection;

pub fn add_test(conn: &mut MysqlConnection, name: &str, email: &str) -> Result<String, String> {
    println!("add_test 함수 호출");
    println!("name: {}", name);
    println!("email: {}", email);
    match infrastructure::create_test(conn, name, email) {
        Ok(_) => Ok(format!("Test with name {} and email {} added successfully.", name, email)),
        Err(e) => Err(format!("Failed to add test: {}", e)),
    }
}

pub fn calculate_sum() -> i32 {
    1 + 1
}