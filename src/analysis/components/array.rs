use crate::analysis::{types::{TilStreamingInterface, TilSignal, Generic, GenericType, StreamDim, TilStreamDirection}, GeneratorParams, gen_tools::type_manager::StreamType};

use super::{Array, JsonComponent, Generatable, JsonComponentValue};

impl Array {
    pub fn new(outer_nested: usize, inner_nested: usize, value: Option<Box<JsonComponent>>) -> Array {
        Array {
            outer_nested,
            inner_nested,
            value,
        }
    }
}

impl Generatable for Array {
    fn get_streaming_interface(&self, gen_params: &GeneratorParams) -> TilStreamingInterface {       
        let mut interface = TilStreamingInterface::new();

        interface.add_generic(Generic::new("EPC", GenericType::Positive(gen_params.epc)));
        let dim_name = "OUTER_NESTING_LEVEL";
        interface.add_generic(Generic::new(dim_name, GenericType::Dimensionality(self.outer_nested)));
        interface.add_generic(Generic::new("INNER_NESTING_LEVEL", GenericType::Natural(self.inner_nested)));
        

        // Input type
        interface.add_stream("input", TilStreamDirection::Input,
            StreamType::Json( 
                StreamDim::new(Some(dim_name.to_string()), self.outer_nested, 1)
            )
        );

        // Output type
        interface.add_stream("output", TilStreamDirection::Output,
            StreamType::Json( 
                StreamDim::new(Some(dim_name.to_string()), self.outer_nested, 2)
            )
        );

        interface
    }

    fn get_streaming_types(&self) -> Vec<StreamType> {
        vec![StreamType::Json(StreamDim::new(None, 0, 0))]
    }

    fn get_preffered_name(&self) -> String {
        "array_parser".to_string()
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

impl JsonComponentValue for Array {
    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("Array parser\nO: {}, I: {}", self.outer_nested, self.inner_nested)
        )
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        match &self.value {
            Some(child) => vec![*child.clone()],
            None => vec![],
        }
    }

    fn num_children(&self) -> usize {
        match &self.value {
            Some(_) => 1,
            None => 0,
        }
    }
}