use crate::webforms::mail::MailData;
use actix_web::HttpResponse;
use actix_web::web::Json;
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::ApiError;
use crate::establish_connection;
use crate::responses::response::{ResponseState, State};
use crate::schema::mails;
use crate::webforms::userformlogin::UserFormLogin;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::mails)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Mail {
    pub id: String,
    pub sender_email: String,
    pub receiver_email: String,
    pub time_sent: NaiveDateTime,
    pub title: String,
    pub content: String,
}

impl Mail{

    pub fn create(mail_data: Json<MailData>) -> Result<ResponseState, ApiError> {
        let query = diesel::insert_into(mails::table)
            .values(Self::from(mail_data))
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

    pub fn get_user_mails(user_data: Json<UserFormLogin>) -> Result<HttpResponse, ApiError> {
        let mails: Vec<Mail> = mails::table.load::<Mail>(&mut establish_connection())?;
        let mail_filtered: Vec<&Mail> = mails
            .iter()
            .filter(|m| m.receiver_email == user_data.email_form)
            .collect::<Vec<&Mail>>();
        Ok(HttpResponse::Ok().json(mail_filtered))
    }

    pub fn from(mail_data: Json<MailData>) -> Mail {
        Mail {
            id: Uuid::new_v4().to_string(),
            sender_email: mail_data.sender_email.to_string(),
            receiver_email: mail_data.receiver_email.to_string(),
            time_sent: mail_data.time_sent,
            title: mail_data.title.to_string(),
            content: mail_data.content.to_string(),
        }
    }
}
