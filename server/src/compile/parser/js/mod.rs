use super::Parser;

mod resolve_module;

/// JS 解析器
pub struct JSParser;

impl Parser for JSParser {
    fn gen_to_code(&self, schema: &crate::models::schema::Schema) -> String {
        let mut code = String::new();

        code.push_str("/*\n\tauthor: xxx\n\tdate: xx\n\tdescript: xxx\n*/\n");

        code.push_str(&format!("const {} = () => {{}};\n", schema.title));

        code.push_str(&format!(
            r"const setup = async () => {{
    try {{
        await {}();
    }} catch (error) {{
        throw error;
    }};
}};
setup();",
            schema.title
        ));

        code
    }
}
