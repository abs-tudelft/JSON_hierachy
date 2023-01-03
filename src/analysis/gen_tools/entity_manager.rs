use std::collections::HashMap;

use indoc::formatdoc;

use crate::analysis::{types::{TilComponent, TilImplementationType}, components::Generatable, GeneratorParams};

use super::{EntityManager, NameReg, TypeManager};

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            entity_list: Vec::new(),
            name_reg: NameReg::new(),
        }
    }

    pub fn register(&mut self, json_component: &&dyn Generatable, gen_params: &GeneratorParams, type_manager: &mut TypeManager, weight: usize) -> TilComponent {
        let name = json_component.get_preffered_name();
        let nesting_level = json_component.get_nesting_level();


        // Get a name for the component
        let registered_name = self.name_reg.register(&name, nesting_level);

        // Create a new component
        let mut entity = TilComponent::new(&registered_name);

        // Generate streaming interface
        let stream_interface = json_component.get_streaming_interface(gen_params);

        // Add interface to component
        entity.set_streaming_interface(stream_interface);

        // Add types to type manager
        for stream_type in json_component.get_streaming_types() {
            type_manager.register(stream_type);
        }

        // Set implementation path
        entity.set_implementation(TilImplementationType::Path("./vhdl_dir".to_string()));

        self.insert_entity(&registered_name, entity.clone(), weight);

        entity
    }

    fn insert_entity(&mut self, name: &str, entity: TilComponent, weight: usize) {
        // Check if index weight exists in entity list
        if self.entity_list.len() <= weight {
            // Create a new entity list
            self.entity_list.push(HashMap::new());
        }

        // Register entity
        self.entity_list[weight].insert(name.to_string(), entity);
    }

    pub fn register_top(&mut self, entity: TilComponent, weight: usize) {
        self.insert_entity("top", entity, weight);
    }

    pub fn generate_stream_defs(&self) -> String{
        let mut stream_defs = String::new();

        for list in &self.entity_list {
            for entity in list.values() {
                stream_defs.push_str(&entity.to_string());

                // Check if there is an implementation
                if let Some(implementation) = entity.get_implementation() {
                    stream_defs.push_str(&self.generate_implementation(implementation));
                }

                stream_defs.push_str(";\n\n")
            }
        }

        stream_defs
    }

    fn generate_implementation(&self, implementation: &TilImplementationType) -> String {
        let mut til = String::new();

        til.push_str(
            &match implementation {
                TilImplementationType::Inline(inline) => {
                    let mut impl_til = String::new();

                    for instance in inline.get_instances() {
                        impl_til.push_str(&instance.to_til());
                        impl_til.push('\n');
                    }

                    impl_til.push('\n');

                    for signal in inline.get_signals() {
                        impl_til.push_str(&signal.to_til());
                        impl_til.push('\n');
                    }

                    
                    formatdoc!(
                        "
                        {{
                            impl: {{
                                {}
                            }}
                        }}",
                        impl_til
                    )
                },
                TilImplementationType::Path(path) => {
                    formatdoc!(
                        "
                        {{
                            impl: \"{}\"
                        }}",
                        path
                    )
                }
            }
        );

        til
    }
}