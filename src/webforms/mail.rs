use chrono::prelude::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct MailData {
    pub sender_email: String,
    pub receiver_email: String,
    pub time_sent: NaiveDateTime,
    pub title: String,
    pub content: String,
}
