use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
pub mod database; // src/infrastructure/mod.rs
use crate::schema::tests; // schema 모듈에서 tests 테이블을 가져옵니다.

#[derive(Insertable)]
#[diesel(table_name = tests)]
pub struct NewTest<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

pub fn create_test(conn: &mut MysqlConnection, name: &str, email: &str) -> Result<String, diesel::result::Error> {
    let new_test = NewTest {
        name,
        email,
    };

    diesel::insert_into(tests::table)
        .values(&new_test)
        .execute(conn)?; // conn이 &mut MysqlConnection이어야 함

    Ok(format!("Test with name {} and email {} added successfully.", name, email))
}

// pub fn get_user_by_id(conn: &mut MysqlConnection, id: i32) -> Result<String, diesel::result::Error> {
//     let user = diesel::select(tests::table.find(id)).first(conn)?;
//     Ok(format!("User with id {} found: {}", id, user.name))
// }

// pub fn update_user(conn: &mut MysqlConnection, id: i32, name: &str, email: &str) -> Result<String, diesel::result::Error> {
//     diesel::update(tests::table.find(id))
//         .set((tests::name.eq(name), tests::email.eq(email)))
//         .execute(conn)?;
//     Ok(format!("User with id {} updated successfully.", id))
// }

// pub fn delete_user(conn: &mut MysqlConnection, id: i32) -> Result<String, diesel::result::Error> {
//     diesel::delete(tests::table.find(id)).execute(conn)?;
//     Ok(format!("User with id {} deleted successfully.", id))
// }