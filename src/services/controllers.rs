use crate::schema::bank_users::dsl::bank_users;
use crate::responses::response::{State, ResponseState};
use crate::webforms::{userformlogin::UserFormLogin, userformregister::UserFormRegister};
use crate::models::User;
use crate::database::database::establish_connection;
use actix_web::{post, web::Json, Responder};
use uuid::Uuid;
use chrono::{Datelike, Utc};

#[post("/api/v1/user/login")]
pub async fn login_user(user_data: Json<UserFormLogin>) -> impl Responder {
    let connection = &mut establish_connection();
    let user: User = bank_users::table
        .select(User::as_select())
        .filter(bank_users::email.eq(&user_data.email_form))
        .first(connection)  
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

#[post("/api/v1/user/register")]
pub async fn add_user(user_data: Json<UserFormRegister>) -> impl Responder {
    let connection = &mut establish_connection();
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: user_data.username_form.to_string(),
        email: user_data.email_form.to_string(),
        dateofbirth: user_data.date_form,
        age: Utc::now().year() - user_data.date_form.year(),
        password: user_data.password_form.to_string(),
    };
    let query = diesel::insert_into(bank_users::table)
        .values(&user)
        .execute(connection);
    let mut query_res = ResponseState {
        state: State::EMPTY,
    };

    match query {
        Ok(_) => query_res.state = State::SUCCESS,
        Err(_) => query_res.state = State::DUPLICATE,
    };
    Json(query_res)
}
