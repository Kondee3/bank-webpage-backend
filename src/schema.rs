// @generated automatically by Diesel CLI.

diesel::table! {
    bank_users (id) {
        id -> Varchar,
        username -> Varchar,
        email -> Varchar,
        dateofbirth -> Date,
        password -> Varchar,
    }
}

diesel::table! {
    mails (id) {
        id -> Varchar,
        sender_email -> Varchar,
        receiver_email -> Varchar,
        time_sent -> Timestamp,
        title -> Varchar,
        content -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bank_users,
    mails,
);
