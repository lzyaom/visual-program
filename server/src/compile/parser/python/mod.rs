use super::Parser;

pub struct PythonParser;

impl Parser for PythonParser {
    fn gen_to_code(&self, _schema: &crate::models::schema::Schema) -> String {
        let mut code = String::new();

        code.push_str(&format!(
            r#"import sys

def trace_calls(frame, event, arg):
    if event == 'call':
        code = frame.f_code
        func_name = code.co_name
        line_no = frame.f_lineno
        print(f"Tracing call to {{func_name}} at line {{line_no}}")
    return trace_lines

def trace_lines(frame, event, arg):
    if event == 'line':
        line_no = frame.f_lineno
        print(f"Tracing line {{line_no}}")
    return trace_lines

def main():
    sys.settrace(trace_calls)
    # Your code to be traced
    print("Hello, world!")
    result = add(1, 2)
    print(f"Result of add: {{result}}")

def add(a, b):
    return a + b

if __name__ == "__main__":
    main()        
"#,
        ));
        code
    }

    fn get_extend(&self) -> String {
        String::from("py")
    }
}
