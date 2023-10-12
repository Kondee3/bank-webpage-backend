use crate::errors::ApiError;
use crate::webforms::{mail::MailData, userformlogin::UserFormLogin};
use actix_web::{web::ServiceConfig, post, web::Json, HttpResponse};
use crate::mail::mail::Mail;

#[post("/api/v1/user/sendmail")]
pub async fn send_mail(mail_data: Json<MailData>) -> Result<HttpResponse, ApiError> {
let response = Mail::create(mail_data)?;   
    Ok(HttpResponse::Ok().json(response))
}

#[post("/api/v1/user/getmails")]
pub async fn get_user_mails(user_data: Json<UserFormLogin>) -> Result<HttpResponse, ApiError> {
let response = Mail::get_user_mails(user_data)?;   
    Ok(response)
}

pub fn init(cfg: &mut ServiceConfig){
    cfg.service(get_user_mails);
    cfg.service(send_mail);
}
