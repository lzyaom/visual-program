use jsonschema::JSONSchema;
use serde_json::Value;
use std::collections::HashMap;

/// 通过程序指令的 `schema` 中定义的规则校验程序内容是否正确
pub fn check_content<'a>(schema: &'a Value, data: &Value) -> Result<bool, Vec<HashMap<String, String>>> {
    let compiler: JSONSchema = JSONSchema::compile(schema).expect("json schema");

    let result = compiler.validate(data);

    match result {
        Err(error) => {
            let info: Vec<_> = error
                .map(|item| {
                    let mut map = HashMap::new();

                    map.insert("path".to_string(), item.instance_path.to_string());

                    map.insert("error".to_string(), item.to_string());

                    map
                })
                .collect();

            Err(info)
        }
        Ok(_) => Ok(true),
    }
}
