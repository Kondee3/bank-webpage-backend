// @generated automatically by Diesel CLI.

diesel::table! {
    bank_users (id) {
        id -> Varchar,
        username -> Varchar,
        email -> Varchar,
        dateofbirth -> Varchar,
        age -> Int4,
        password -> Varchar,
    }
}
