use crate::domain;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use crate::schema::tests;

#[derive(Insertable)]
#[diesel(table_name = tests)]
pub struct NewTest<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

pub fn create_test(conn: &mut MysqlConnection, name: &str, email: &str) -> Result<String, String> {
    let new_test = NewTest {
        name,
        email,
    };

    // diesel::insert_into(tests::table)
    //     .values(&new_test)
    //     .execute(conn)?;

    // Ok(format!("Test with name {} and email {} added successfully.", name, email))

    domain::add_test(conn, name, email)

}

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
