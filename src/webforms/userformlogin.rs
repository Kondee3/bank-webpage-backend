use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserFormLogin {
    pub email_form: String,
    pub password_form: String,
}
