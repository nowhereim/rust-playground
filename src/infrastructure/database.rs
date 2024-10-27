use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use dotenvy::dotenv;
use std::env;


// pub fn establish_connection() -> MysqlConnection {
//     // 환경 변수에서 DATABASE_URL을 가져옵니다.
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
    
//     // 데이터베이스 연결을 설정합니다.
//     MysqlConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }


// pub fn establish_connection() -> MysqlConnection {
//     // 하드코딩된 데이터베이스 URL
//     let database_url = "mysql://root:rootpassword@127.0.0.1:3306/playground";
    
//     // 데이터베이스 연결을 설정합니다.
//     MysqlConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok(); // .env 파일 로드
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}