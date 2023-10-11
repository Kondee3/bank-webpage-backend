use serde::{Serialize, Deserialize};
use chrono::prelude::NaiveDate;

#[derive(Serialize, Deserialize)]
pub struct UserFormRegister {
    pub username_form: String,
    pub password_form: String,
    pub email_form: String,
    pub date_form: NaiveDate,
}
