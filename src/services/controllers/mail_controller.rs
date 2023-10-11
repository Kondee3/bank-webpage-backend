use crate::establish_connection;
use crate::models::convert_mail;
use crate::models::Mail;
use crate::errors::ApiError;
use crate::responses::response::{ResponseState, State};
use crate::schema::mails;
use crate::webforms::{mail::MailData, userformlogin::UserFormLogin};
use actix_web::{post, web::Json, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::SelectableHelper;
 
#[post("/api/v1/user/sendmail")]
pub async fn send_mail(mail_data: Json<MailData>) -> impl Responder {
    let query = diesel::insert_into(mails::table)
        .values(convert_mail(mail_data))
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
#[post("/api/v1/user/getmails")]
pub async fn fetch_mails(user_data: Json<UserFormLogin>) -> Result<HttpResponse, ApiError>{
    //let mail= mails::table
     //   .select(Mail::as_select())
      //  .filter(mails::receiver_email.eq(&user_data.email_form))
      //  .first(&mut establish_connection())
      //  .load(&mut establish_connection)?;
    let mail: Vec<Mail> = mails::table
        .load::<Mail>(&mut establish_connection())?;
    let mail_filtered: Vec<&Mail>= mail.iter()
        .filter(|m| m.receiver_email == user_data.email_form)
        .collect::<Vec<&Mail>>();
    Ok(HttpResponse::Ok().json(mail_filtered))
        
}
