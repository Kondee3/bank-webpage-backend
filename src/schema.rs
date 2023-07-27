// @generated automatically by Diesel CLI.

diesel::table! {
    bank_users (id) {
        id -> Int4,
        user_name -> Varchar,
        email -> Varchar,
        date_of_birth -> Nullable<Date>,
        age -> Nullable<Int4>,
    }
}
