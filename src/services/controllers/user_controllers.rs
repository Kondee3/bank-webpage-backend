use crate::errors::ApiError;
use crate::user::user::User;
use crate::webforms::{userformregister::UserFormRegister, userformlogin::UserFormLogin};
use actix_web::{web::ServiceConfig, post, web::Json, HttpResponse};


#[post("/api/v1/user/register")]
pub async fn register(user_data: Json<UserFormRegister>) -> Result<HttpResponse, ApiError> {
    let response = User::register(user_data)?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/api/v1/user/login")]
pub async fn login(user_data: Json<UserFormLogin>) -> Result<HttpResponse, ApiError> {
    let response = User::login(user_data)?;
    Ok(HttpResponse::Ok().json(response))
}

pub fn init(cfg: &mut ServiceConfig){
    cfg.service(register);
    cfg.service(login);
}
