use super::{visualization, Generator, GeneratorParams, analyzer::Analyzer};

impl Generator {
    pub fn new(project_name: &str, epc: usize, bit_width: usize, int_width: usize) -> Generator {
        Generator {
            root: None,
            analyzer: Analyzer::new(),
            gen_params: GeneratorParams::new(epc, bit_width, int_width, "", project_name),
        }
    }

    // Analyze a JSON string
    pub fn analyze(&mut self, json: &str) -> Result<(), GeneratorError> {
        // Deserialize the JSON string
        let parsed = json::parse(json)
        // In case of error, return the error
        .map_err(GeneratorError::JsonError)?; 

        self.analyzer.analyze(&parsed, self.gen_params.clone());

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

        // Set the output directory
        self.gen_params.output_dir = String::from(dir);

        // Create the directory if it doesn't exist
        std::fs::create_dir_all(dir).unwrap();

        // Create the file
        let mut file = std::fs::File::create(path).unwrap();

        use std::io::Write;

        let til = self.generate_til();

        file.write_fmt(format_args!("{}", til)).unwrap();

        // Generate the files
        self.analyzer.generate_files(&self.gen_params.output_dir);

        Ok(())
    }
}

#[derive(Debug)]
pub enum GeneratorError {
    JsonError(json::JsonError),
    NoRoot,
}