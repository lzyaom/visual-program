use crate::models::schema::Schema;

pub fn parse_to_python_code(_schema: Schema) -> String {
    let mut code = String::new();

    code.push_str(&format!("def __init__(self):"));

    code
}

pub fn parse_to_js_code(schema: &Schema) -> String {
    let mut code = String::new();

    code.push_str("/*\n\tauthor: xxx\n\tdate: xx\n\tdescript: xxx\n*/\n");

    code.push_str(&format!("const {} = () => {{}};\n", schema.title));

    code.push_str(&format!(
        r"
const setup = async () => {{
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
