use crate::analysis::{GeneratorParams, types::{TilStreamType, Synchronicity, TilStreamingInterface}, gen_tools::TypeManager};

use super::{Record, JsonComponent, Generatable, Key, JsonComponentValue};

impl Record {
    pub fn new(outer_nested: usize, inner_nested: usize, key: Key) -> Record {
        Record {
            outer_nested,
            inner_nested,
            key
        }
    }
}

impl Generatable for Record {
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

        // Output type
        let output_type = TilStreamType::new(
            &format!("{}OutStream", component_name),
            gen_params.bit_width,
            gen_params.epc,
            self.outer_nested + 2,
            Synchronicity::Sync,
            8,
        );

        type_manager.register(output_type.clone());
        interface.add_output_stream("output", output_type);

        interface
    }

    fn get_preffered_name(&self) -> String {
        "record_parser".to_string()
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

impl JsonComponentValue for Record {
    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("Record parser\nO: {}, I: {}", self.outer_nested, self.inner_nested)
        )
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        vec![JsonComponent::Key(self.key.clone())]
    }

    fn num_children(&self) -> usize {
        1
    }
}