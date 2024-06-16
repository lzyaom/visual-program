use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ProgramStatus {
    Draft = 0,
    InProcess = 1,
    Done = 2,
    Share = 3,
}
// todo 实现多人协同程序文件
#[derive(Debug, Deserialize, Serialize)]
pub struct Program {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub status: ProgramStatus,
    pub descript: String,
    pub create_time: String,
}

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct ProgramQuery {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: ProgramStatus,
    pub page: i64,
    pub size: i64,
}
