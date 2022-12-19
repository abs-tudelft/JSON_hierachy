use crate::analysis::{GeneratorParams, gen_tools::TypeManager, types::{TilStreamType, Synchronicity, TilStreamingInterface}};

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
    fn get_streaming_interface(&self, component_name: &str, gen_params: &GeneratorParams, type_manager: &mut TypeManager) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::new();

        // Type generation
        // Input type
        let input_type = TilStreamType::new(
            &format!("{}InStream", component_name),
            gen_params.bit_width,
            gen_params.epc,
            self.outer_nested + 1,
            Synchronicity::Sync,
            8,
        );

        type_manager.register(input_type.clone());
        interface.add_input_stream("input", input_type);

        // Matcher type
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
        interface.add_input_stream("matcherIn", matcher_type.clone());
        interface.add_output_stream("matcherOut", matcher_type);

        // Output type
        let output_type = TilStreamType::new(
            &format!("{}OutStream", component_name),
            gen_params.bit_width,
            gen_params.epc,
            self.outer_nested + 1,
            Synchronicity::Sync,
            8,
        );

        type_manager.register(output_type.clone());
        interface.add_output_stream("output", output_type);        

        interface
    }

    fn get_preffered_name(&self) -> String {
        "key_parser".to_string()
    }

    fn get_nesting_level(&self) -> usize {
        self.outer_nested
    }

    // fn to_til_signal(&self, component_name: &str, parent_name: &str) -> Option<String> {
    //     Some(
    //         formatdoc!(
    //             "
    //             {}.output -- {}.input;
    //             ",
    //             parent_name,
    //             component_name,
    //         )
    //     )
    // }

    // fn to_til_top_input_signal(&self, component_name: &str, top_input_name: &str) -> Option<String> {
    //     Some(
    //         formatdoc!(
    //             "
    //             {} -- {}.input;
    //             ",
    //             top_input_name,
    //             component_name,
    //         )
    //     )
    // }

    // fn to_til_top_output_signal(&self, component_name: &str, top_output_name: &str) -> Option<String> {
    //     Some(
    //         formatdoc!(
    //             "
    //             {}.output -- {};
    //             ",
    //             component_name,
    //             top_output_name,
    //         )
    //     )
    // }
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