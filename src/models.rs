use diesel::prelude::*;
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::bank_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct User{
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub date_of_birth: String,
    pub age: i32,
}
