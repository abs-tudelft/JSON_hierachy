use crate::analysis::{types::{TilStreamingInterface, TilSignal, streaming_interface::{Generic, GenericType, TilStreamDirection, TilStream}, stream_types::{StreamTypeDecl, StreamDim}}, GeneratorParams, analyzer::type_manager::StreamType};

use super::{JsonComponent, JsonType, Value, Generatable, JsonComponentValue};

impl Value {
    pub fn new(name: &str, data_type: JsonType, outer_nested: usize) -> Value {
        Value {
            name: name.to_string(),
            data_type,
            outer_nested,
        }
    }
}

impl Generatable for Value {
    fn get_streaming_interface(&self, gen_params: &GeneratorParams) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::default();
        interface.add_generic(Generic::new("EPC", GenericType::Positive(gen_params.epc)));
        let dim_name = "DIM";
        let dim = self.outer_nested + 1;
        interface.add_generic(Generic::new(dim_name, GenericType::Dimensionality(dim)));
        interface.add_generic(Generic::new("NESTING_LEVEL", GenericType::Positive(self.outer_nested)));

        // Input type
        interface.add_stream("input", TilStreamDirection::Input,
            StreamTypeDecl::new(
                StreamType::Json,
                Some(StreamDim::new(Some(dim_name.to_string()), dim, 0))
            )
        );

        match self.data_type {
            JsonType::String => {
                // Output type
                interface.add_stream("output", TilStreamDirection::Output,
                    StreamTypeDecl::new(
                        StreamType::Json, 
                        Some(StreamDim::new(Some(dim_name.to_string()),  dim, 0))
                    )
                );

                interface
            },
            JsonType::Integer => {
                interface.add_generic(Generic::new("BITWIDTH", GenericType::Positive(gen_params.int_width)));

                // Output type
                interface.add_stream("output", TilStreamDirection::Output,
                    StreamTypeDecl::new(
                        StreamType::Int, 
                        Some(StreamDim::new(Some(dim_name.to_string()),  dim, -1))
                    )
                );

                interface
            },
            JsonType::Boolean => {
                // Output type
                interface.add_stream("output", TilStreamDirection::Output,
                    StreamTypeDecl::new(
                        StreamType::Bool,
                        Some(StreamDim::new(Some(dim_name.to_string()),  dim, -1))
                    )
                );

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

    fn get_nesting_level(&self) -> usize {
        self.outer_nested
    }

    fn get_outgoing_signals(&self) -> Vec<TilSignal> {
        let output_name = format!("output_{}", self.get_instance_name());

        vec![
            TilSignal::Output { 
                source_inst_name: self.get_instance_name(), 
                source_stream_name: "output".to_owned(), 
                dest_stream_name: output_name.clone(),
                output_stream: TilStream::new(&output_name, TilStreamDirection::Output, 
                    match self.data_type {
                        JsonType::String => StreamTypeDecl::new(StreamType::Json, Some(StreamDim::new(None, self.outer_nested, 1))),
                        JsonType::Integer => StreamTypeDecl::new(StreamType::Int, Some(StreamDim::new(None, self.outer_nested, 0))),
                        JsonType::Boolean => StreamTypeDecl::new(StreamType::Bool, Some(StreamDim::new(None, self.outer_nested, 0))),
                    }
                )
            }
        ]
    }

    fn get_name(&self) -> &str {
        &self.name
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