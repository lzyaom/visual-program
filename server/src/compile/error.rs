/// 用于处理可能出现的错误情况
/// 1. 文件类
/// 2. 校验类
/// 3. 解析类
/// 4. 生成类
/// 5. 通讯类
/// 6. 程序内部错误
/// 7. 运行类
pub enum CompileError {
    FileTypeError,
    ParamsError,
    ContentNoneError,
    ParseError,
}
