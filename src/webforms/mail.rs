
use serde::{Serialize, Deserialize};
use chrono::prelude::NaiveDateTime;
#[derive(Serialize, Deserialize)]
pub struct MailData{
    pub sender_email: String,
    pub receiver_email: String,
    pub time_sent: NaiveDateTime,
    pub title: String,
    pub content: String,
}
