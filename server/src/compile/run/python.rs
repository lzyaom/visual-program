use pyo3::prelude::*;
use pyo3::types::PyModule;

pub fn run(source_code: &str) -> PyResult<()> {
    // 初始化 python 解析器
    Python::with_gil(|py| {
        let module = PyModule::from_code_bound(py, &source_code, "runtime.py", "runtime")?;

        // 调用函数
        module.call_method0("main")?;
        Ok(())
    })
}
