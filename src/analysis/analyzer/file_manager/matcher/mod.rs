use crate::analysis::generator::GeneratorError;

pub fn generate_matcher(matcher: &str) -> Result<String, GeneratorError> {
    use pyo3::prelude::*;
    
    let code = include_str!("./vhdre/vhdre/__init__.py");

    // Create the Python interpreter
    Python::with_gil(|py| -> PyResult<String> {
        let vhdre = PyModule::from_code(py, code, "vhdre/__init__.py", "vhdre")?;

        let regex_class = vhdre.getattr("RegexMatcher")?;
        let regex = regex_class.call1((format!("{}_matcher", matcher), matcher))?;

        let vhdl: &str = regex.call_method0("__str__")?.extract::<&str>()?;

        Ok(String::from(vhdl))
    }).map_err(GeneratorError::PythonError)
}