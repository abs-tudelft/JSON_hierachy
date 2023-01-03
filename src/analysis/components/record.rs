use crate::analysis::{types::{TilStreamingInterface, TilSignal, GenericType, Generic, StreamDim, TilStreamDirection}, gen_tools::{type_manager::StreamType}, GeneratorParams};

use super::{Record, JsonComponent, Generatable, Key, JsonComponentValue};

impl Record {
    pub fn new(outer_nested: usize, inner_nested: usize, key: Key) -> Record {
        Record {
            outer_nested,
            inner_nested,
            key
        }
    }
}

impl Generatable for Record {
    fn get_streaming_interface(&self, gen_params: &GeneratorParams) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::new();

        interface.add_generic(Generic::new("EPC", GenericType::Positive(gen_params.epc)));
        let dim_name = "DIM";
        let dim = self.outer_nested + 1;
        interface.add_generic(Generic::new(dim_name, GenericType::Dimensionality(dim)));
        interface.add_generic(Generic::new("OUTER_NESTING_LEVEL", GenericType::Positive(self.outer_nested)));
        interface.add_generic(Generic::new("INNER_NESTING_LEVEL", GenericType::Natural(self.inner_nested)));

        // Input type
        interface.add_stream("input", TilStreamDirection::Input,
            StreamType::Json( 
                StreamDim::new(Some(dim_name.to_string()), dim, 0)
            )
        );

        // Output type
        interface.add_stream("output", TilStreamDirection::Output, 
            StreamType::Record( 
                StreamDim::new(Some(dim_name.to_string()), dim, 1)
            )
        );

        interface
    }

    fn get_streaming_types(&self) -> Vec<StreamType> {
        vec![StreamType::Json(StreamDim::new(None, 0, 0)), StreamType::Record(StreamDim::new(None, 0, 0))]
    }

    fn get_preffered_name(&self) -> String {
        "record_parser".to_string()
    }

    fn get_nesting_level(&self) -> usize {
        self.outer_nested
    }

    fn get_signals(&self, instance_name: &Option<String>, instance_stream_name: &str, parent_name: &Option<String>, parent_stream_name: &str) -> Vec<TilSignal> {
        vec![TilSignal::new(parent_name, parent_stream_name, instance_name, instance_stream_name)]     
    }

    fn num_outgoing_signals(&self) -> usize {
        1
    }
}

impl JsonComponentValue for Record {
    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("Record parser\nO: {}, I: {}", self.outer_nested, self.inner_nested)
        )
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        vec![JsonComponent::Key(self.key.clone())]
    }

    fn num_children(&self) -> usize {
        1
    }
}