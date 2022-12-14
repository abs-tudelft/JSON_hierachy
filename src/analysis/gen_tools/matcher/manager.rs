use crate::analysis::GeneratorParams;

use super::MatcherManager;

impl MatcherManager {
    pub fn new() -> MatcherManager {
        MatcherManager {
            matchers: Vec::new()
        }
    }

    pub fn add_matcher(&mut self, matcher: &String, gen_params: &GeneratorParams) {
        // Check if the matcher is already in the list
        if !self.matchers.contains(matcher) {
            self.matchers.push(matcher.to_string());
            self.generate_matcher(matcher, gen_params);
        }
    }

    fn generate_matcher(&self, matcher: &str, gen_params: &GeneratorParams) {
        use pyo3::prelude::*;
        
        let code = include_str!("./vhdre/vhdre/__init__.py");

        // Create the Python interpreter
        match Python::with_gil(|py| -> PyResult<String> {
            let vhdre = PyModule::from_code(py, &code, "vhdre/__init__.py", "vhdre")?;

            let regex_class = vhdre.getattr("RegexMatcher")?;
            let regex = regex_class.call1((format!("{}_matcher", matcher), matcher))?;

            let vhdl: &str = regex.call_method0("__str__")?.extract::<&str>()?;

            Ok(String::from(vhdl))
        }) {
            Ok(vhdl) => {
                let matchers_dir = format!("{}/matchers", gen_params.output_dir);

                // Create the matchers directory if it doesn't exist
                std::fs::create_dir_all(&matchers_dir).unwrap();

                // Write the VHDL code to a file
                let mut file = std::fs::File::create(format!("{}/{}_matcher.vhd", &matchers_dir, &matcher)).unwrap();

                use std::io::Write;
                file.write_fmt(format_args!("{}", vhdl)).unwrap();
            },
            Err(e) => println!("{:?}", &e),
        }
    }
}