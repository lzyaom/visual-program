pub mod js;
pub mod python;

use crate::models::schema::Schema;

/// 解析器 Trait
pub trait Parser {
    /// 获取扩展符
    fn get_extend(&self) -> String {
        String::from("js")
    }

    /// 生成代码
    fn gen_to_code(&self, schema: &Schema) -> String;
}
