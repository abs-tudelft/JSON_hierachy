use crate::analysis::{GeneratorParams, types::{TilStreamingInterface, TilSignal}, gen_tools::{TypeManager, type_manager::StreamType}};

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
        // Register the matcher type
        type_manager.register(StreamType::MatcherStr);
        interface.add_input_stream("input", StreamType::MatcherStr);

        // Register the matcher type
        type_manager.register(StreamType::MatcherMatch);
        interface.add_output_stream("output", StreamType::MatcherStr);

        interface
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

    // fn get_input_type_params(&self, gen_params: &GeneratorParams) -> TilStreamParam {
    //     TilStreamParam::new(
    //         gen_params.bit_width,
    //         gen_params.epc,
    //         1,
    //         Synchronicity::Sync,
    //         8
    //     )
    // }

    // fn get_output_type_params(&self, gen_params: &GeneratorParams) -> TilStreamParam {
    //     TilStreamParam::new(
    //         1,
    //         gen_params.epc,
    //         1,
    //         Synchronicity::Sync,
    //         8
    //     )
    // }
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