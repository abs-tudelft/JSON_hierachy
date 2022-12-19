use crate::analysis::{GeneratorParams, gen_tools::TypeManager, types::{TilStreamType, Synchronicity, TilStreamingInterface}};

use super::{JsonComponent, JsonType, Value, Generatable, JsonComponentValue};

impl Value {
    pub fn new(data_type: JsonType, outer_nested: usize) -> Value {
        Value {
            data_type,
            outer_nested,
        }
    }
}

impl Generatable for Value {
    fn get_streaming_interface(&self, component_name: &str, gen_params: &GeneratorParams, type_manager: &mut TypeManager) -> TilStreamingInterface {
        match self.data_type {
            JsonType::String => {
                let mut interface = TilStreamingInterface::new();

                // Type generation
                // Input type
                let input_type = TilStreamType::new(
                    &format!("{}InStream", component_name),
                    gen_params.bit_width,
                    gen_params.epc,
                    self.outer_nested + 1,
                    Synchronicity::Sync,
                    8,
                );

                type_manager.register(input_type.clone());
                interface.add_input_stream("input", input_type);

                // Output type
                let output_type = TilStreamType::new(
                    &format!("{}OutStream", component_name),
                    gen_params.bit_width,
                    gen_params.epc,
                    self.outer_nested + 1,
                    Synchronicity::Sync,
                    8,
                );

                type_manager.register(output_type.clone());
                interface.add_output_stream("output", output_type);

                interface
            },
            JsonType::Integer => {
                let mut interface = TilStreamingInterface::new();

                // Type generation
                // Input type
                let input_type = TilStreamType::new(
                    &format!("{}InStream", component_name),
                    gen_params.bit_width,
                    gen_params.epc,
                    self.outer_nested + 1,
                    Synchronicity::Sync,
                    8,
                );

                type_manager.register(input_type.clone());
                interface.add_input_stream("input", input_type);

                // Output type
                let output_type = TilStreamType::new(
                    &format!("{}OutStream", component_name),
                    gen_params.int_width,
                    1,
                    self.outer_nested,
                    Synchronicity::Sync,
                    2,
                );

                type_manager.register(output_type.clone());
                interface.add_output_stream("output", output_type);

                interface
            },
            JsonType::Boolean => {
                let mut interface = TilStreamingInterface::new();

                // Type generation
                // Input type
                let input_type = TilStreamType::new(
                    &format!("{}InStream", component_name),
                    gen_params.bit_width,
                    gen_params.epc,
                    self.outer_nested + 1,
                    Synchronicity::Sync,
                    8,
                );

                type_manager.register(input_type.clone());
                interface.add_input_stream("input", input_type);

                // Output type
                let output_type = TilStreamType::new(
                    &format!("{}OutStream", component_name),
                    1,
                    1,
                    self.outer_nested,
                    Synchronicity::Sync,
                    2,
                );

                type_manager.register(output_type.clone());
                interface.add_output_stream("output", output_type);

                interface
            }
        }
    }

    fn get_preffered_name(&self) -> String {
        match self.data_type {
            JsonType::String => "string_parser".to_string(),
            JsonType::Integer => "int_parser".to_string(),
            JsonType::Boolean => "bool_parser".to_string(),
        }
    }

    fn get_nesting_level(&self) -> usize {
        self.outer_nested
    }

    // fn to_til_signal(&self, component_name: &str, parent_name: &str) -> Option<String> {
    //     Some(
    //         formatdoc!(
    //             "
    //             {}.output -- {}.input;
    //             ",
    //             parent_name,
    //             component_name,
    //         )
    //     )
    // }

    // fn to_til_top_input_signal(&self, component_name: &str, top_input_name: &str) -> Option<String> {
    //     Some(
    //         formatdoc!(
    //             "
    //             {} -- {}.input;
    //             ",
    //             top_input_name,
    //             component_name,
    //         )
    //     )
    // }

    // fn to_til_top_output_signal(&self, component_name: &str, top_output_name: &str) -> Option<String> {
    //     Some(
    //         formatdoc!(
    //             "
    //             {}.output -- {};
    //             ",
    //             component_name,
    //             top_output_name,
    //         )
    //     )
    // }
}

impl JsonComponentValue for Value {
    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("{:?} parser\nO: {}", self.data_type, self.outer_nested)
        )
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        Vec::new()
    }

    fn num_children(&self) -> usize {
        0
    }
}