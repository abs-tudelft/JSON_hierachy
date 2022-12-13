use super::{visualization, components, Generator, vhdl, til, NameReg, GeneratorParams};

impl Generator {
    pub fn new() -> Generator {
        Generator {
            root: None
        }
    }

    // Analyze a JSON string
    pub fn analyze(&mut self, json: &str) -> Result<(), GeneratorError> {
        // Deserialize the JSON string
        let parsed = json::parse(json)
        // In case of error, return the error
        .map_err(|e| GeneratorError::JsonError(e))?; 

        self.root = components::analyze(&parsed);

        Ok(())
    }

    // Visualize the component tree as a dot file
    pub fn visualize(&self, path: &str) -> Result<(), GeneratorError> {
        // Check if the root exists
        if let Some(root) = &self.root {
            visualization::generate_dot(&root, path)
        } else { return Err(GeneratorError::NoRoot); }

        Ok(())
    }

    // Generate VHDL code from the analyzed JSON string
    pub fn vhdl(&self, path: &str) -> Result<(), GeneratorError> {
        // Separate output path into directory and file name
        let (dir, _) = path.split_at(path.rfind('/').unwrap_or(0));

        // Create the directory if it doesn't exist
        std::fs::create_dir_all(dir).unwrap();

        // Create the file
        let mut file = std::fs::File::create(path).unwrap();

        use std::io::Write;

        // Write the prelude
        file.write_fmt(format_args!("{}", vhdl::generate_prelude())).unwrap();

        if let Some(root) = &self.root {
            

            file.write_fmt(format_args!("{}", root.to_vhdl())).unwrap();
        } else { return Err(GeneratorError::NoRoot); }

        // Write the postlude
        file.write_fmt(format_args!("{}", vhdl::generate_postlude())).unwrap();

        Ok(())
    }

    pub fn til(&self, path: &str) -> Result<(), GeneratorError> {
        // Separate output path into directory and file name
        let (dir, _) = path.split_at(path.rfind('/').unwrap_or(0));

        // Create the directory if it doesn't exist
        std::fs::create_dir_all(dir).unwrap();

        // Create the file
        let mut file = std::fs::File::create(path).unwrap();

        use std::io::Write;

        let mut file_buffer = String::new();

        // Write the prelude
        file_buffer.push_str(&til::generate_prelude());

        if let Some(root) = &self.root {
            let mut name_reg = NameReg::new();

            let gen_params = GeneratorParams {
                epc: 2,
                bit_width: 8,
                int_width: 64,
            };

            let mut til = root.to_til(&mut name_reg, &gen_params);

            // Insert \t at the beginning of each line
            til = til.replace("\n", "\n\t");
            // Remove the last \t
            til.pop();

            file_buffer.push_str(&til);
        } else { return Err(GeneratorError::NoRoot); }

        // Write the postlude
        file_buffer.push_str(&til::generate_postlude());

        file.write_fmt(format_args!("{}", file_buffer)).unwrap();

        Ok(())
    }
}

#[derive(Debug)]
pub enum GeneratorError {
    JsonError(json::JsonError),
    NoRoot,
}