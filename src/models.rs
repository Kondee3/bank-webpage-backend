
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::bank_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct User{
    pub id: String,
    pub username: String,
    pub email: String,
    pub dateofbirth:NaiveDate,
    pub age: i32,
    pub password: String,
}
