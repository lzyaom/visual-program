use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    account: String,
    psw: String,
    salt: String,
    nickname: String,
    avatar: String,
}
