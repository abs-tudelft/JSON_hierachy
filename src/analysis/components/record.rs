use crate::analysis::{types::{TilStreamingInterface, TilSignal, streaming_interface::{Generic, GenericType, TilStreamDirection}, stream_types::{StreamTypeDecl, StreamDim}}, GeneratorParams, analyzer::{type_manager::StreamType, file_manager::TemplateType}};

use super::{Record, JsonComponent, Generatable, Key, JsonComponentValue};

impl Record {
    pub fn new(name: &str, outer_nested: usize, inner_nested: usize, key: Key) -> Record {
        Record {
            name: name.to_string(),
            outer_nested,
            inner_nested,
            key
        }
    }
}

impl Generatable for Record {
    fn get_streaming_interface(&self, gen_params: &GeneratorParams) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::default();

        interface.add_generic(Generic::new("EPC", GenericType::Positive(gen_params.epc)));
        let dim_name = "OUTER_NESTING_LEVEL";
        interface.add_generic(Generic::new(dim_name, GenericType::Dimensionality(self.outer_nested)));
        interface.add_generic(Generic::new("INNER_NESTING_LEVEL", GenericType::Natural(self.inner_nested)));

        // Input type
        interface.add_stream("input", TilStreamDirection::Input,
            StreamTypeDecl::new(
                StreamType::Json,
                Some(StreamDim::new(Some(dim_name.to_string()), self.outer_nested, 1))
            )
        );

        // Output type
        interface.add_stream("output", TilStreamDirection::Output,
            StreamTypeDecl::new(
                StreamType::Record,
                Some(StreamDim::new(Some(dim_name.to_string()), self.outer_nested, 2))
            )
        );

        interface
    }

    fn get_streaming_types(&self) -> Vec<StreamType> {
        vec![StreamType::Json, StreamType::Record]
    }

    fn get_nesting_level(&self) -> usize {
        self.outer_nested
    }

    fn get_outgoing_signals(&self) -> Vec<TilSignal> {
        vec![
            TilSignal::Intermediate { 
                source_inst_name: self.get_instance_name(), 
                source_stream_name: "output".to_owned(), 
                dest_inst_name: self.key.get_instance_name(), 
                dest_stream_name: "input".to_owned() 
            }
        ]
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn num_outgoing_signals(&self) -> usize {
        1
    }

    fn get_file_type(&self) -> TemplateType {
        TemplateType::Record
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