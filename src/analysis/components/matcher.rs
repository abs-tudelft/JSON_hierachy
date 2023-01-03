use crate::analysis::{types::{TilStreamingInterface, TilSignal, Generic, GenericType, TilStreamDirection}, gen_tools::{type_manager::StreamType}, GeneratorParams};

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
    fn get_streaming_interface(&self, gen_params: &GeneratorParams) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::new();

        interface.add_generic(Generic::new("BPC", GenericType::Positive(gen_params.epc)));

        // Input type
        interface.add_stream("input", TilStreamDirection::Input, StreamType::MatcherStr);

        // Output type
        interface.add_stream("output", TilStreamDirection::Output, StreamType::MatcherMatch);

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