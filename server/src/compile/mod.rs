pub mod check;
pub mod error;
mod parser;
mod run;

use dotenv::dotenv;
use parser::{python::PythonParser, Parser};
use std::{env, fs, path::Path};

pub struct Compiler {
    pub file_name: String,
    pub parser: Box<dyn Parser>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            file_name: String::from("runtime"),
            parser: Box::new(PythonParser {}),
        }
    }

    pub fn set_file_name(self, file_name: String) -> Self {
        Self { file_name, ..self }
    }

    /// 创建可运行程序文件 [`Program`]
    pub fn create_file(&self, code: &str) -> Result<bool, String> {
        dotenv().ok();

        let dir = env::var("RUNTIME_FILE_DIR".to_string()).expect(&format!("read RUNTIME_FILE_DIR env variable fail"));

        let dir_state = fs::read_dir(&dir);

        if let Err(_) = dir_state {
            let _ = fs::create_dir(&dir);
        }

        let path = format!("./{dir}/{}.{}", self.file_name, self.parser.get_extend());

        let file_path = Path::new(&path);

        let result = fs::write(file_path, code);

        if let Err(error) = result {
            return Err(format!(
                "write content to {}.{} file fail, error: {error}",
                self.file_name,
                self.parser.get_extend()
            ));
        }

        Ok(true)
    }

    pub fn run(&self, source_code: &str) -> Result<String, String> {
        // run::python::run(source_code)
        let result = run::python::run(&source_code);
        match result {
            Err(e) => Err(e.to_string()),
            _ => Ok("success".to_string()),
        }
    }
}
