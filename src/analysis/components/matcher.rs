use crate::analysis::{types::{TilStreamingInterface, TilSignal}, gen_tools::{type_manager::StreamType}};

use super::{JsonComponent, Matcher, Generatable, JsonComponentValue};

impl Matcher {
    pub fn new(matcher: String, outer_nested: usize) -> Matcher {
        Matcher {
            matcher,
            outer_nested
        }
    }

    pub fn get_matcher(&self) -> &str {
        &self.matcher
    }
}

impl Generatable for Matcher {
    fn get_streaming_interface(&self) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::new();

        // Input type
        interface.add_input_stream("input", StreamType::MatcherStr);

        // Output type
        interface.add_output_stream("output", StreamType::MatcherMatch);

        interface
    }

    fn get_streaming_types(&self) -> Vec<StreamType> {
        vec![StreamType::MatcherStr, StreamType::MatcherMatch]
    }

    fn get_preffered_name(&self) -> String {
        format!("{}_matcher", self.matcher)
    }

    fn get_nesting_level(&self) -> usize {
        self.outer_nested
    }

    fn get_signals(&self, instance_name: &Option<String>, _instance_stream_name: &str, parent_name: &Option<String>, _parent_stream_namee: &str) -> Vec<TilSignal> {
        vec![
            TilSignal::new(parent_name, "matcher_str", instance_name, "input"),
            TilSignal::new(instance_name, "output", parent_name, "matcher_match"),
        ]     
    }

    fn num_outgoing_signals(&self) -> usize {
        1
    }
}

impl JsonComponentValue for Matcher {
    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("Regex matcher\n\"{}\"", self.matcher)
        )   
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        Vec::new()
    }

    fn num_children(&self) -> usize {
        0
    }
}