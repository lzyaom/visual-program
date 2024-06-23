use super::Parser;

pub struct PythonParser;

impl Parser for PythonParser {
    fn gen_to_code(&self, schema: &crate::models::schema::Schema) -> String {
        let mut code = String::new();

        code.push_str(&format!("class {}:\n", schema.title));

        code.push_str(&format!("\tdef __init__(self):\n"));

        code.push_str(&format!("\ndef main():\n"));
        code
    }

    fn get_extend(&self) -> String {
        String::from("py")
    }
}
