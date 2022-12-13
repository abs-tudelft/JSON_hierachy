use super::{visualization, Generator, NameReg, GeneratorParams, analyzer};

impl Generator {
    pub fn new(epc: u16, bit_width: u16, int_width: u16) -> Generator {
        Generator {
            root: None,
            name_map: NameReg::new(),
            gen_params: GeneratorParams {
                epc,
                bit_width,
                int_width,
            },
        }
    }

    // Analyze a JSON string
    pub fn analyze(&mut self, json: &str) -> Result<(), GeneratorError> {
        // Deserialize the JSON string
        let parsed = json::parse(json)
        // In case of error, return the error
        .map_err(|e| GeneratorError::JsonError(e))?; 

        self.root = analyzer::analyze(&parsed);

        Ok(())
    }

    // Visualize the component tree as a dot file
    pub fn visualize(&self, path: &str) -> Result<(), GeneratorError> {
        // Check if the root exists
        match &self.root {
            Some(root) => {
                visualization::generate_dot(root, path)
            },
            None => return Err(GeneratorError::NoRoot),
        }

        Ok(())
    }

    pub fn generate(&mut self, path: &str) -> Result<(), GeneratorError> {
        // Separate output path into directory and file name
        let (dir, _) = path.split_at(path.rfind('/').unwrap_or(0));

        // Create the directory if it doesn't exist
        std::fs::create_dir_all(dir).unwrap();

        // Create the file
        let mut file = std::fs::File::create(path).unwrap();

        use std::io::Write;

        let til = self.generate_til();

        file.write_fmt(format_args!("{}", til)).unwrap();

        Ok(())
    }
}

#[derive(Debug)]
pub enum GeneratorError {
    JsonError(json::JsonError),
    NoRoot,
}