use super::{visualization, Generator, GeneratorParams, analyzer::Analyzer, GeneratorError};

impl Generator {
    pub fn new(project_name: &str, epc: usize, bit_width: usize, int_width: usize) -> Generator {
        Generator {
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
        let root = self.analyzer.get_root().map_err(GeneratorError::AnalyzerError)?;

        visualization::generate_dot(root, path);

        Ok(())
    }

    pub fn generate(&mut self, path: &str) -> Result<(), GeneratorError> {
        // Set the output directory
        self.gen_params.output_dir = format!("{}/{}", path, self.gen_params.project_name);
        let proj_dir = &self.gen_params.output_dir;

        // Check if directory exists
        if std::path::Path::new(proj_dir).exists() {
            // If it does, delete it
            std::fs::remove_dir_all(proj_dir).unwrap();
        }

        // Create the directory if it doesn't exist
        std::fs::create_dir_all(format!("{}/src", proj_dir)).unwrap();

        // Create the file
        let mut file = std::fs::File::create(format!("{}/src/{}.til", proj_dir, self.gen_params.project_name)).unwrap();

        use std::io::Write;

        let til = self.generate_til();

        file.write_fmt(format_args!("{}", til)).unwrap();

        // Generate the files
        let file_manager = self.analyzer.get_file_manager();
        file_manager.generate_files(&self.gen_params.output_dir, &self.gen_params);

        file_manager.generate_toml(&self.gen_params.output_dir, &self.gen_params);

        Ok(())
    }
}