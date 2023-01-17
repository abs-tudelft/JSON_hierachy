use json::JsonValue;

use crate::analysis::components::JsonComponent;

use self::file_manager::FileManager;

use super::{types::{TilComponent, TilSignal}, GeneratorParams, analyzer::{name_reg::NameReg, type_manager::{TypeManager, StreamType}}};

mod analysis;
mod name_reg;
pub mod type_manager;
pub mod top_component;
pub mod file_manager;

/**********************************************************************************
 * Set of functions to analyze the parsed JSON object into a component structure  *
 * which can be used to generate HDL code.                                        *
 **********************************************************************************/

pub struct Analyzer {
    name_reg: NameReg,
    type_manager: TypeManager,
    entity_list: Vec<TilComponent>,
    file_manager: FileManager,
    gen_params: GeneratorParams,
    signal_list: Vec<TilSignal>,
    top_component: Option<JsonComponent>,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        Analyzer {
            name_reg: NameReg::new(),
            type_manager: TypeManager::new(),
            entity_list: Vec::new(),
            file_manager: FileManager::new(),
            gen_params: GeneratorParams::default(),
            signal_list: Vec::new(),
            top_component: None,
        }
    }

    pub fn analyze(&mut self, root: &JsonValue, gen_params: GeneratorParams) {
        self.gen_params = gen_params;

        let (root_component, _) = self.analyze_element(root, 0, 0);
        self.top_component = root_component;
    }

    pub fn get_definitions(&self) -> (Vec<StreamType>, &Vec<TilComponent>) {   
        let stream_types = self.type_manager.get_stream_types();
        let til_components = &self.entity_list;
        
        (stream_types, til_components)
    }

    pub fn get_file_manager(&self) -> &FileManager {
        &self.file_manager
    }
}

#[derive(Debug)]
pub enum AnalyzerError {
    NoTop,
    PythonError(String),
}