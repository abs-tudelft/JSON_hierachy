use crate::analysis::{gen_tools::{type_manager::StreamType}, types::{TilStreamingInterface, TilSignal}};

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
    fn get_streaming_interface(&self) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::new();

        match self.data_type {
            JsonType::String => {
                // Input type
                interface.add_input_stream("input", StreamType::Json);

                // Output type
                interface.add_output_stream("output", StreamType::Json);

                interface
            },
            JsonType::Integer => {
                // Input type
                interface.add_input_stream("input", StreamType::Json);

                // Output type
                interface.add_output_stream("output", StreamType::Int);

                interface
            },
            JsonType::Boolean => {
                // Input type
                interface.add_input_stream("input", StreamType::Json);

                // Output type
                interface.add_output_stream("output", StreamType::Bool);

                interface
            }
        }
    }

    fn get_streaming_types(&self) -> Vec<StreamType> {
        match self.data_type {
            JsonType::String => vec![StreamType::Json],
            JsonType::Integer => vec![StreamType::Json, StreamType::Int],
            JsonType::Boolean => vec![StreamType::Json, StreamType::Bool],
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

    fn get_signals(&self, instance_name: &Option<String>, instance_stream_name: &str, parent_name: &Option<String>, parent_stream_name: &str) -> Vec<TilSignal> {
        vec![TilSignal::new(parent_name, parent_stream_name, instance_name, instance_stream_name)]     
    }

    fn num_outgoing_signals(&self) -> usize {
        0
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