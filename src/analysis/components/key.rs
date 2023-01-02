use crate::analysis::{gen_tools::{type_manager::StreamType}, types::{TilStreamingInterface, TilSignal}};

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
    fn get_streaming_interface(&self) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::new();

        // Input type
        interface.add_input_stream("input", StreamType::Record);

        // Matcher type
        interface.add_output_stream("matcher_str", StreamType::MatcherStr);
        interface.add_input_stream("matcher_match", StreamType::MatcherMatch);

        // Output type
        interface.add_output_stream("output", StreamType::Json);        

        interface
    }

    fn get_streaming_types(&self) -> Vec<StreamType> {
        vec![StreamType::Record, StreamType::MatcherStr, StreamType::MatcherMatch, StreamType::Json]
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