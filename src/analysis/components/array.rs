use crate::analysis::{GeneratorParams, types::{TilStreamingInterface, TilSignal}, gen_tools::{TypeManager, type_manager::StreamType}};

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
    fn get_streaming_interface(&self, component_name: &str, gen_params: &GeneratorParams, type_manager: &mut TypeManager) -> TilStreamingInterface {       
        let mut interface = TilStreamingInterface::new();

        // Generate types for this component
        type_manager.register(StreamType::Json);

        // Input type
        interface.add_input_stream("input", StreamType::Json);

        // Output type
        interface.add_output_stream("output", StreamType::Json);

        interface
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

    // fn get_input_type_params(&self, gen_params: &GeneratorParams) -> StreamType {
    //     TilStreamParam::new(
    //         gen_params.bit_width,
    //         gen_params.epc,
    //         self.outer_nested + 1,
    //         Synchronicity::Sync,
    //         8,
    //     )
    // }

    // fn get_output_type_params(&self, gen_params: &GeneratorParams) -> StreamType {
    //     TilStreamParam::new(
    //         gen_params.bit_width,
    //         gen_params.epc,
    //         self.outer_nested + 2,
    //         Synchronicity::Sync,
    //         8,
    //     )
    // }
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