use dotenv::dotenv;
use std::env;
use std::fs;
use std::path::Path;

/// 创建可运行程序文件 [`Program`]
pub fn create_run_file(code: String, file_type: String) -> Result<bool, String> {
    dotenv().ok();

    let dir = env::var("RUNTIME_FILE_DIR".to_string()).expect(&format!("read RUNTIME_FILE_DIR env variable fail"));

    let file_name =
        env::var("RUNTIME_FILE_NAME".to_string()).expect(&format!("read RUNTIME_FILE_NAME env variable fail"));

    let dir_state = fs::read_dir(&dir);

    if let Err(_) = dir_state {
        let _ = fs::create_dir(&dir);
    }

    let path = format!("./{dir}/{file_name}.{file_type}");

    let file_path = Path::new(&path);

    let result = fs::write(file_path, code);

    if let Err(_error) = result {
        return Err(format!("create runtime.{file_type} file fail"));
    }

    Ok(true)
}
