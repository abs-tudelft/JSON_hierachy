use self::{components::JsonComponent, analyzer::Analyzer};

mod components;
mod visualization;
pub mod generator;
pub mod analyzer;
pub mod types;
// mod vhdl;
mod til;


pub struct Generator {
    root: Option<JsonComponent>,
    analyzer: Analyzer,
    gen_params: GeneratorParams,
}

#[derive(Default, Clone)]
pub struct GeneratorParams {
    epc: usize,
    bit_width: usize,
    int_width: usize,
    output_dir: String,
    project_name: String,
    namespace: String,
    comp_namespace: String,
}

impl GeneratorParams {
    pub fn new(epc: usize, bit_width: usize, int_width: usize, output_dir: &str, project_name: &str) -> Self {
        let project_name = validate_project_name(project_name).unwrap();

        let (til_ns, comp_ns) = namespace_from_project_name(&project_name);
        
        GeneratorParams {
            epc,
            bit_width,
            int_width,
            output_dir: output_dir.to_owned(),
            project_name,
            namespace: til_ns,
            comp_namespace: comp_ns
        }
    }
}

fn validate_project_name(project_name: &str) -> Result<String, GeneratorError> {
    // Check if project name is invalid from the following criteria:
    // 1. Project name is empty
    // 2. Project name contains a space
    // 3. Project name cannot contain any of the following characters: \ / : * ? " < > | , . ; ' ! @ # $ % ^ & ( ) - + = [ ] { } ` ~
    // 4. Project cannot contain consecutive underscores
    // 5. Project cannot start or end with an underscore or number

    // 1
    if project_name.is_empty() {
        return Err(GeneratorError::InvalidProjectName);
    }

    // 2 & 3
    for c in project_name.chars() {
        if c == ' ' || c == '\\' || c == '/' || c == ':' || c == '*' || c == '?' || c == '"' || c == '<' || c == '>' || c == '|' || c == ',' || c == '.' || c == ';' || c == '\'' || c == '!' || c == '@' || c == '#' || c == '$' || c == '%' || c == '^' || c == '&' || c == '(' || c == ')' || c == '-' || c == '+' || c == '=' || c == '[' || c == ']' || c == '{' || c == '}' || c == '`' || c == '~' {
            return Err(GeneratorError::InvalidProjectName);
        }
    }

    // 4
    if project_name.contains("__") {
        return Err(GeneratorError::InvalidProjectName);
    }

    // 5
    if project_name.starts_with('_') || project_name.starts_with('0') || project_name.starts_with('1') || project_name.starts_with('2') || project_name.starts_with('3') || project_name.starts_with('4') || project_name.starts_with('5') || project_name.starts_with('6') || project_name.starts_with('7') || project_name.starts_with('8') || project_name.starts_with('9') {
        return Err(GeneratorError::InvalidProjectName);
    }

    Ok(project_name.to_string())
}

fn namespace_from_project_name(project_name: &str) -> (String, String) {
    let mut til_namespace = String::new();
    let mut comp_namespace = String::new();

    for c in project_name.chars() {
        if c == '_' {
            til_namespace.push_str("::");
            comp_namespace.push_str("_0_");
        } else {
            til_namespace.push(c);
            comp_namespace.push(c);
        }
    }

    (til_namespace, comp_namespace)
}

#[derive(Debug)]
pub enum GeneratorError {
    NoRoot,
    InvalidProjectName,
}