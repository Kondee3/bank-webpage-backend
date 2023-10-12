use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserFormLogin {
    pub email_form: String,
    pub password_form: String,
}
