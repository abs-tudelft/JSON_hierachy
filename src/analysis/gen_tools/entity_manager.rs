use std::collections::HashMap;

use indoc::formatdoc;

use crate::analysis::{types::TilComponent, components::Generatable, GeneratorParams};

use super::{EntityManager, NameReg, TypeManager};

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            entity_list: HashMap::new(),
            name_reg: NameReg::new(),
        }
    }

    pub fn register(&mut self, json_component: &&dyn Generatable, gen_params: &GeneratorParams, type_manager: &mut TypeManager) -> TilComponent {
        let name = json_component.get_preffered_name();
        let nesting_level = json_component.get_nesting_level();

        // Get a name for the component
        let registered_name = self.name_reg.register(&name, nesting_level);

        // Create a new component
        let mut entity = TilComponent::new(&registered_name);

        // Generate streaming interface
        let stream_interface = json_component.get_streaming_interface(&registered_name, gen_params, type_manager);

        // Add interface to component
        entity.set_streaming_interface(stream_interface);

        // Register entity
        self.entity_list.insert(registered_name, entity.clone());

        entity
    }

    pub fn get_entity(&self, name: &str) -> Option<&TilComponent> {
        self.entity_list.get(name)
    }

    pub fn generate_stream_defs(&self) -> String{
        let mut stream_defs = String::new();

        for (name, entity) in &self.entity_list {
            let mut local_stream_defs = String::new();

            for stream in entity.streams.get_input_streams() {
                local_stream_defs.push_str(
                    &format!("{}: in {},\n", stream.get_name(), stream.get_type().name)
                );
            }

            for stream in entity.streams.get_output_streams() {
                local_stream_defs.push_str(
                    &format!("{}: out {},\n", stream.get_name(), stream.get_type().name)
                );
            }

            stream_defs.push_str(
                &formatdoc!(
                    "
                    streamlet {} = (
                        {}
                    );
                    ",
                    name,
                    local_stream_defs
                )
            );

            stream_defs.push('\n');
        }

        stream_defs
    }
}