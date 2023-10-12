use crate::errors::ApiError;
use crate::establish_connection;
use crate::responses::response::{ResponseState, State};
use crate::schema::bank_users;
use crate::webforms::{userformlogin::UserFormLogin, userformregister::UserFormRegister};
use actix_web::web::Json;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::bank_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub dateofbirth: NaiveDate,
    pub password: String,
}
impl User {
    pub fn register(user_data: Json<UserFormRegister>) -> Result<ResponseState, ApiError> {
        let query = diesel::insert_into(bank_users::table)
            .values(Self::from(user_data))
            .execute(&mut establish_connection());
        let mut query_res = ResponseState {
            state: State::EMPTY,
        };
        match query {
            Ok(_) => query_res.state = State::SUCCESS,
            Err(_) => query_res.state = State::DUPLICATE,
        };
        Ok(query_res)
    }
    pub fn from(user_data: Json<UserFormRegister>) -> User {
        User {
            id: Uuid::new_v4().to_string(),
            username: user_data.username_form.to_string(),
            email: user_data.email_form.to_string(),
            dateofbirth: user_data.date_form,
            password: user_data.password_form.to_string(),
        }
    }
    pub fn login(user_data: Json<UserFormLogin>) -> Result<ResponseState, ApiError> {
        let user: User = bank_users::table
            .select(User::as_select())
            .filter(bank_users::email.eq(&user_data.email_form))
            .first(&mut establish_connection())
            .expect("error");
        let mut query_res = ResponseState {
            state: State::EMPTY,
        };
        query_res.state = if &user.password == &user_data.password_form {
            State::SUCCESS
        } else if user.password.is_empty() {
            State::NOTFOUND
        } else {
            State::WRONGPASSWORD
        };
        Ok(query_res)
    }
}
