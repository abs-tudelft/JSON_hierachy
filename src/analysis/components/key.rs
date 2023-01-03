use crate::analysis::{gen_tools::{type_manager::StreamType}, types::{TilStreamingInterface, TilSignal, Generic, GenericType, StreamDim, TilStreamDirection}, GeneratorParams};

use super::{Key, Generatable, JsonComponent, Matcher, JsonComponentValue};

impl Key {
    pub fn new(matcher: Matcher, outer_nested: usize, value: Option<Box<JsonComponent>>) -> Key {
        Key {
            matcher,
            outer_nested,
            value
        }
    }
}

impl Generatable for Key {
    fn get_streaming_interface(&self, gen_params: &GeneratorParams) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::new();

        interface.add_generic(Generic::new("EPC", GenericType::Positive(gen_params.epc)));
        let dim_name = "OUTER_NESTING_LEVEL";
        interface.add_generic(Generic::new(dim_name, GenericType::Dimensionality(self.outer_nested)));

        // Input type
        interface.add_stream("input", TilStreamDirection::Input, 
            StreamType::Record( 
                StreamDim::new(Some(dim_name.to_string()), self.outer_nested, 1)
            )
        );

        // Matcher type
        interface.add_stream("matcher_str", TilStreamDirection::Output, StreamType::MatcherStr);
        interface.add_stream("matcher_match", TilStreamDirection::Input, StreamType::MatcherMatch);

        // Output type
        interface.add_stream("output", TilStreamDirection::Output,
            StreamType::Json( 
                StreamDim::new(Some(dim_name.to_string()), self.outer_nested, 1)
            )
        );        

        interface
    }

    fn get_streaming_types(&self) -> Vec<StreamType> {
        vec![StreamType::Record(StreamDim::new(None, 0, 0)), StreamType::MatcherStr, StreamType::MatcherMatch, StreamType::Json(StreamDim::new(None, 0, 0))]
    }

    fn get_preffered_name(&self) -> String {
        "key_parser".to_string()
    }

    fn get_nesting_level(&self) -> usize {
        self.outer_nested
    }

    fn get_signals(&self, instance_name: &Option<String>, instance_stream_name: &str, parent_name: &Option<String>, parent_stream_name: &str) -> Vec<TilSignal> {
        vec![TilSignal::new(parent_name, parent_stream_name, instance_name, instance_stream_name)]     
    }

    fn num_outgoing_signals(&self) -> usize {
        2
    }
}

impl JsonComponentValue for Key {
    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("Key filter\nO: {}", self.outer_nested)
        )
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        match &self.value {
            Some(child) => vec![JsonComponent::Matcher(self.matcher.clone()), *child.clone()],
            None => vec![JsonComponent::Matcher(self.matcher.clone())],
        }
    }

    fn num_children(&self) -> usize {
        match &self.value {
            Some(_) => 2,
            None => 1,
        }
    }
}