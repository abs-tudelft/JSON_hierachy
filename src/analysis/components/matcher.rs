use crate::analysis::{types::{TilStreamingInterface, TilSignal, Generic, GenericType, TilStreamDirection, stream_types::StreamTypeDecl}, GeneratorParams, analyzer::type_manager::StreamType};

use super::{JsonComponent, Matcher, Generatable, JsonComponentValue};

impl Matcher {
    pub fn new(name: &str, holder_name: &str, matcher: String, outer_nested: usize) -> Matcher {
        Matcher {
            name: name.to_string(),
            holder_name: holder_name.to_string(),
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
        let mut interface = TilStreamingInterface::default();

        interface.add_generic(Generic::new("BPC", GenericType::Positive(gen_params.epc)));

        // Input type
        interface.add_stream("input", TilStreamDirection::Input, 
            StreamTypeDecl::new(
                StreamType::MatcherStr,
                None
            )
        );

        // Output type
        interface.add_stream("output", TilStreamDirection::Output,  
            StreamTypeDecl::new(
                StreamType::MatcherMatch,
                None
            )
        );

        interface
    }

    fn get_streaming_types(&self) -> Vec<StreamType> {
        vec![StreamType::MatcherStr, StreamType::MatcherMatch]
    }

    fn get_nesting_level(&self) -> usize {
        self.outer_nested
    }

    fn get_outgoing_signals(&self) -> Vec<TilSignal> {
        vec![TilSignal::new(Some(self.name.to_string()), "output", Some(self.holder_name.to_string()), "matcher_match")]
    }

    fn num_outgoing_signals(&self) -> usize {
        1
    }

    fn get_name(&self) -> &str {
        &self.name
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