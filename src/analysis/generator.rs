use super::{visualization, components, Generator};

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

        if let Some(root) = &self.root {
            use std::io::Write;

            file.write_fmt(format_args!("{}", root.to_vhdl())).unwrap();
        } else { return Err(GeneratorError::NoRoot); }

        Ok(())
    }
}

#[derive(Debug)]
pub enum GeneratorError {
    JsonError(json::JsonError),
    NoRoot,
}