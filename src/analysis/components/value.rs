use crate::analysis::{GeneratorParams, gen_tools::TypeManager, types::{TilStreamType, Synchronicity, TilStreamingInterface, TilSignal, TilStreamParam}};

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
                    self.get_input_type_params(gen_params)
                );

                type_manager.register(input_type.clone());
                interface.add_input_stream("input", input_type);

                // Output type
                let output_type = TilStreamType::new(
                    &format!("{}OutStream", component_name),
                    self.get_output_type_params(gen_params)
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
                    self.get_input_type_params(gen_params)
                );

                type_manager.register(input_type.clone());
                interface.add_input_stream("input", input_type);

                // Output type
                let output_type = TilStreamType::new(
                    &format!("{}OutStream", component_name),
                    self.get_output_type_params(gen_params)
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
                    self.get_input_type_params(gen_params)
                );

                type_manager.register(input_type.clone());
                interface.add_input_stream("input", input_type);

                // Output type
                let output_type = TilStreamType::new(
                    &format!("{}OutStream", component_name),
                    self.get_output_type_params(gen_params)
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

    fn get_signals(&self, instance_name: &Option<String>, parent_name: &Option<String>) -> Vec<TilSignal> {
        vec![TilSignal::new(parent_name, "output", instance_name, "input")]
    }

    fn num_outgoing_signals(&self) -> usize {
        0
    }

    fn get_input_type_params(&self, gen_params: &GeneratorParams) -> TilStreamParam {
        TilStreamParam::new(
            gen_params.bit_width, 
            gen_params.epc, 
            self.outer_nested + 1, 
            Synchronicity::Sync,
            8
        )
    }

    fn get_output_type_params(&self, gen_params: &GeneratorParams) -> TilStreamParam {
        match self.data_type {
            JsonType::String => TilStreamParam::new(
                gen_params.bit_width,
                gen_params.epc,
                self.outer_nested + 1,
                Synchronicity::Sync,
                8,
            ),
            JsonType::Integer => TilStreamParam::new(
                gen_params.int_width,
                1,
                self.outer_nested,
                Synchronicity::Sync,
                2,
            ),
            JsonType::Boolean => TilStreamParam::new(
                1,
                1,
                self.outer_nested,
                Synchronicity::Sync,
                2,
            ),
        }
    }
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