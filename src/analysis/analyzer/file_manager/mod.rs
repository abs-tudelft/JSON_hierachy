use std::{fs::File, collections::HashMap, io::Write};

use enum_map::{EnumMap, Enum, enum_map};
use text_template::Template;

mod matcher;

use crate::analysis::GeneratorParams;

#[derive(Enum, Clone, Copy, Debug)]
pub enum TemplateType {
    Array,
    Int,
    Bool,
    Record,
    Key,
    String,
    Matcher
}

struct TemplateInstance {
    pub template_type: TemplateType,
    pub component_name: String,
}

pub(super) struct FileManager {
    files: Vec<TemplateInstance>,
    templates: EnumMap<TemplateType, Option<String>>,
}

impl FileManager {
    pub fn new() -> Self {
        FileManager {
            files: Vec::new(),
            templates: enum_map! {
                TemplateType::Array => Some(String::from(include_str!("templates/array_parser.vhd"))),
                TemplateType::Int => Some(String::from(include_str!("templates/int_parser.vhd"))),
                TemplateType::Bool => None,
                TemplateType::Record => Some(String::from(include_str!("templates/record_parser.vhd"))),
                TemplateType::Key => Some(String::from(include_str!("templates/key_parser.vhd"))),
                TemplateType::String => None,
                TemplateType::Matcher => None,
            },
        }
    }

    pub fn add_entity(&mut self, template_type: TemplateType, component_name: &str) {
        self.files.push(TemplateInstance {
            template_type,
            component_name: component_name.to_owned(),
        });
    }

    pub fn generate_files(&self, output_path: &str, gen_params: &GeneratorParams) {
        // Create the directory if it doesn't exist
        let path = format!("{}/{}", output_path, "vhdl_dir");
        std::fs::create_dir_all(&path).unwrap();

        // Generate the files
        for inst in &self.files {
            let file_name = format!("{}/{}_0_{}.vhd", path, gen_params.comp_namespace, inst.component_name);
            let mut file = File::create(file_name).unwrap();

            let text = self.file_from_template(inst, gen_params);
            file.write_all(text.as_bytes()).unwrap();
        }
    }

    fn file_from_template(&self, template_inst: &TemplateInstance, gen_params: &GeneratorParams) -> String {
        // Get the template
        let template = match template_inst.template_type {
            // Matcher needs to be handled differently as it depends on the matching string
            TemplateType::Matcher => {
                Some(matcher::generate_matcher(&template_inst.component_name).unwrap())
            },
            _ => self.templates[template_inst.template_type].to_owned()
        };

        let template_str = match template {
            Some(template_str) => template_str,
            None => todo!("Template for {:?} not implemented", template_inst.template_type),
        };

        let template = Template::from(template_str.as_str());

        let mut templ_values: HashMap<&str, &str> = HashMap::new();
        templ_values.insert("comp_name", &template_inst.component_name);
        let bit_width = gen_params.bit_width.to_string();
        templ_values.insert("bit_width", &bit_width);
        templ_values.insert("namespace", &gen_params.comp_namespace);
        templ_values.insert("project_name", &gen_params.project_name);
    
        let text = template.fill_in(&templ_values).to_string();
    
        text
    }
}

