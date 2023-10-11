use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use chrono::Utc;
use uuid::Uuid;
use actix_web::web::Json;
use crate::webforms::userformregister::UserFormRegister;
use crate::webforms::mail::MailData;

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

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::mails)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Mail{
    pub id: String,
    pub sender_email: String,
    pub receiver_email: String,
    pub time_sent: NaiveDateTime,
    pub title: String,
    pub content: String,
}

pub fn convert_user(user_data: Json<UserFormRegister>)->User{
     User{
        id: Uuid::new_v4().to_string(),
        username: user_data.username_form.to_string(),
        email: user_data.email_form.to_string(),
        dateofbirth: user_data.date_form,
        age: Utc::now().year() - user_data.date_form.year(),
        password: user_data.password_form.to_string(),
      }
}

pub fn convert_mail(mail_data: Json<MailData>)->Mail{
     Mail{
        id: Uuid::new_v4().to_string(),
        sender_email:mail_data.sender_email.to_string(), 
        receiver_email: mail_data.receiver_email.to_string(),
        time_sent: mail_data.time_sent,
        title: mail_data.title.to_string(),
        content: mail_data.content.to_string(),
      }
}
