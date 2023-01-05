use json::JsonValue;

use crate::analysis::components::JsonComponent;

use super::{types::TilComponent, GeneratorParams, analyzer::{name_reg::NameReg, type_manager::{TypeManager, StreamType}}};

mod analysis;
mod name_reg;
pub mod type_manager;

/**********************************************************************************
 * Set of functions to analyze the parsed JSON object into a component structure  *
 * which can be used to generate HDL code.                                        *
 **********************************************************************************/

pub struct Analyzer {
    name_reg: NameReg,
    type_manager: TypeManager,
    entity_list: Vec<TilComponent>,
    gen_params: GeneratorParams,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        Analyzer {
            name_reg: NameReg::new(),
            type_manager: TypeManager::new(),
            entity_list: Vec::new(),
            gen_params: GeneratorParams::default(),
        }
    }

    pub fn analyze(&mut self, root: &JsonValue, gen_params: GeneratorParams) -> Option<JsonComponent> {
        self.gen_params = gen_params;

        let (root_component, _) = self.analyze_element(root, 0, 0);
        root_component
    }

    pub fn get_definitions(&self) -> (Vec<StreamType>, &Vec<TilComponent>) {   
        let stream_types = self.type_manager.get_stream_types();
        let til_components = &self.entity_list;
        
        (stream_types, til_components)
    }
}

