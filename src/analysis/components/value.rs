use crate::analysis::{gen_tools::{type_manager::StreamType}, types::{TilStreamingInterface, TilSignal, Generic, GenericType, StreamDim, TilStreamDirection}, GeneratorParams};

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
    fn get_streaming_interface(&self, gen_params: &GeneratorParams) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::new();
        interface.add_generic(Generic::new("EPC", GenericType::Positive(gen_params.epc)));
        let dim_name = "NESTING_LEVEL";
        interface.add_generic(Generic::new(dim_name, GenericType::Dimensionality(self.outer_nested)));

        // Input type
        interface.add_stream("input", TilStreamDirection::Input,
            StreamType::Json( 
                StreamDim::new(Some(dim_name.to_string()), self.outer_nested, 1)
            )
        );

        match self.data_type {
            JsonType::String => {
                // Output type
                interface.add_stream("output", TilStreamDirection::Output,
                    StreamType::Json( 
                        StreamDim::new(Some(dim_name.to_string()),  self.outer_nested, 1)
                    )
                );

                interface
            },
            JsonType::Integer => {
                interface.add_generic(Generic::new("BITWIDTH", GenericType::Positive(gen_params.int_width)));

                // Output type
                interface.add_stream("output", TilStreamDirection::Output,
                StreamType::Int( 
                        StreamDim::new(Some(dim_name.to_string()),  self.outer_nested, 0)
                    )
                );

                interface
            },
            JsonType::Boolean => {
                // Output type
                interface.add_stream("output", TilStreamDirection::Output,
                StreamType::Bool( 
                        StreamDim::new(Some(dim_name.to_string()),  self.outer_nested, 0)
                    )
                );

                interface
            }
        }
    }

    fn get_streaming_types(&self) -> Vec<StreamType> {
        match self.data_type {
            JsonType::String => vec![StreamType::Json(StreamDim::new(None, 0, 0))],
            JsonType::Integer => vec![StreamType::Json(StreamDim::new(None, 0, 0)), StreamType::Int(StreamDim::new(None, 0, 0))],
            JsonType::Boolean => vec![StreamType::Json(StreamDim::new(None, 0, 0)), StreamType::Bool(StreamDim::new(None, 0, 0))],
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