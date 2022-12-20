use crate::analysis::{GeneratorParams, types::{TilStreamType, Synchronicity, TilStreamingInterface, TilSignal, TilStreamParam}, gen_tools::TypeManager};

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
    fn get_streaming_interface(&self, _component_name: &str, gen_params: &GeneratorParams, type_manager: &mut TypeManager) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::new();

        // Type generation
        let matcher_type = TilStreamType::new(
            "MatcherStream",
            self.get_input_type_params(gen_params)
        );

        // Register the matcher type
        type_manager.register(matcher_type.clone());
        interface.add_input_stream("input", matcher_type.clone());
        interface.add_output_stream("output", matcher_type);

        interface
    }

    fn get_preffered_name(&self) -> String {
        format!("{}_matcher", self.matcher)
    }

    fn get_nesting_level(&self) -> usize {
        self.outer_nested
    }

    fn get_signals(&self, instance_name: &Option<String>, instance_stream_name: &str, parent_name: &Option<String>, parent_stream_name: &str) -> Vec<TilSignal> {
        vec![
            TilSignal::new(parent_name, "matcherOut", instance_name, "input"),
            TilSignal::new(instance_name, "output", parent_name, "matcherIn"),
        ]     
    }

    fn num_outgoing_signals(&self) -> usize {
        1
    }

    fn get_input_type_params(&self, gen_params: &GeneratorParams) -> TilStreamParam {
        TilStreamParam::new(
            1,
            gen_params.epc,
            1,
            Synchronicity::Sync,
            8
        )
    }

    fn get_output_type_params(&self, gen_params: &GeneratorParams) -> TilStreamParam {
        self.get_input_type_params(gen_params)
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