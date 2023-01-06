use crate::analysis::analyzer::AnalyzerError;

pub fn generate_matcher(matcher: &str, comp_name: &str, project_name: &str) -> Result<String, AnalyzerError> {
    use pyo3::prelude::*;
    
    let code = include_str!("./vhdre/vhdre/__init__.py");

    // Create the Python interpreter
    Python::with_gil(|py| -> PyResult<String> {
        let vhdre = PyModule::from_code(py, code, "vhdre/__init__.py", "vhdre")?;

        let regex_class = vhdre.getattr("RegexMatcher")?;
        let regex = regex_class.call1((comp_name, project_name, matcher))?;

        let vhdl: &str = regex.call_method0("__str__")?.extract::<&str>()?;

        Ok(String::from(vhdl))
    }).map_err(|e| AnalyzerError::PythonError(e.to_string()))
}