use crate::analysis::{GeneratorParams, types::{TilStreamType, Synchronicity, TilStreamingInterface}, gen_tools::TypeManager};

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
    fn get_streaming_interface(&self, component_name: &str, gen_params: &GeneratorParams, type_manager: &mut TypeManager) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::new();

        // Type generation
        let matcher_type = TilStreamType::new(
            "MatcherStream",
            1,
            gen_params.epc,
            1,
            Synchronicity::Sync,
            8,
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

    // fn to_til_signal(&self, component_name: &str, parent_name: &str) -> Option<String> {
    //     Some(
    //         formatdoc!(
    //             "
    //             {}.matcherOut -- {}.input;
    //             {}.output -- {}.matcherIn;
    //             ", 
    //             parent_name, 
    //             component_name,
    //             component_name,
    //             parent_name                
    //         )
    //     )
    // }

    // fn to_til_top_input_signal(&self, _component_name: &str, _top_input_name: &str) -> Option<String> {
    //     None
    // }

    // fn to_til_top_output_signal(&self, _component_name: &str, _top_output_name: &str) -> Option<String> {
    //     None
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