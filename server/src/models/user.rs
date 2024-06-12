use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub account: String,
    pub psw: String,
    pub salt: String,
    pub nickname: String,
    pub avatar: String,
}
