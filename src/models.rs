use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use uuid::Uuid;
use actix_web::web::Json;
use crate::webforms::userformregister::UserFormRegister;
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

pub fn convert(user_data: Json<UserFormRegister>)->User{
     User{
        id: Uuid::new_v4().to_string(),
        username: user_data.username_form.to_string(),
        email: user_data.email_form.to_string(),
        dateofbirth: user_data.date_form,
        age: Utc::now().year() - user_data.date_form.year(),
        password: user_data.password_form.to_string(),
      }
}
