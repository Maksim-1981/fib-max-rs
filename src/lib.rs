use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pymodule]
fn fib_max_rs(_py: Python, m: &PyModule) -> \
PyResult<()> {
m.add_wrapped(wrap_pyfunction!(say_hello));
Ok(())
}
