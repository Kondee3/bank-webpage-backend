use crate::establish_connection;
use crate::models::convert;
use crate::models::*;
use crate::responses::response::{ResponseState, State};
use crate::schema::bank_users;
use crate::webforms::{userformlogin::UserFormLogin, userformregister::UserFormRegister};
use actix_web::{post, web::Json, Responder};
use diesel::prelude::*;
use diesel::SelectableHelper;
#[post("/api/v1/user/login")]
pub async fn login(user_data: Json<UserFormLogin>) -> impl Responder {
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
    Json(query_res)
}
