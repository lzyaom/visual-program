use pyo3::prelude::*;
use pyo3::types::PyModule;

pub fn run(_source_code: &str) -> PyResult<()> {
    Python::with_gil(|py| {
        let script = r#"
import trace
import sys

def sample_function():
    print("Hello from Python!")

def another_function():
    print("Another function in Python.")

class ReportTrace(trace.Trace):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

    def localtrace_trace(self, frame, event, arg):
        if event == 'call':
            print(f'Calling {frame.f_code.co_name} at line {frame.f_lineno}')
        elif event == 'line':
            print(f'Executing line {frame.f_lineno} in {frame.f_code.co_name}')
        return self.localtrace_trace

# Create a Trace object, telling it what to ignore, and whether to
# do tracing or line-counting or both.
tracer = ReportTrace(
    ignoredirs=[sys.prefix, sys.exec_prefix],
    trace=True,
    count=False,
)

# Execute the script under the control of the Trace object.
tracer.run('sample_function()')
tracer.run('another_function()')
"#;
        PyModule::from_code_bound(py, &script, "runtime", "")?;

        Ok(())
    })
}
