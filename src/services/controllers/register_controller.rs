use crate::establish_connection;
use crate::models::convert;
use crate::models::*;
use crate::responses::response::{ResponseState, State};
use crate::schema::bank_users;
use crate::webforms::{userformlogin::UserFormLogin, userformregister::UserFormRegister};
use actix_web::{post, web::Json, Responder};
use diesel::prelude::*;
use diesel::SelectableHelper;
#[post("/api/v1/user/register")]
pub async fn register(user_data: Json<UserFormRegister>) -> impl Responder {
    let query = diesel::insert_into(bank_users::table)
        .values(convert(user_data))
        .execute(&mut establish_connection());
    let mut query_res = ResponseState {
        state: State::EMPTY,
    };
    match query {
        Ok(_) => query_res.state = State::SUCCESS,
        Err(_) => query_res.state = State::DUPLICATE,
    };
    Json(query_res)
}
