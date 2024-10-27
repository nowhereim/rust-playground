// @generated automatically by Diesel CLI.

diesel::table! {
    tests (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}
