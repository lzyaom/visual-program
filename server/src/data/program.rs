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
    id: ObjectId,
    title: String,
    status: ProgramStatus,
    descript: String,
    create_time: String,
}
