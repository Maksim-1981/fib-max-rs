use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pymodule]
fn flitton_fib_rs(_py: Python, m: &PyModule) -> \
PyResult<()> {
m.add_wrapped(wrap_pyfunction!(say_hello));
m.add_wrapped(wrap_pyfunction!(fibonacci_number));
m.add_wrapped(wrap_pyfunction!(fibonacci_numbers);
Ok(())
}
