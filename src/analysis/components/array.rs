use crate::analysis::{GeneratorParams, types::{TilStreamType, Synchronicity, TilStreamingInterface}, gen_tools::TypeManager};

use super::{Array, JsonComponent, Generatable, JsonComponentValue};

impl Array {
    pub fn new(outer_nested: usize, inner_nested: usize, value: Option<Box<JsonComponent>>) -> Array {
        Array {
            outer_nested,
            inner_nested,
            value,
        }
    }
}

impl Generatable for Array {
    fn get_streaming_interface(&self, component_name: &str, gen_params: &GeneratorParams, type_manager: &mut TypeManager) -> TilStreamingInterface {       
        let mut interface = TilStreamingInterface::new();

        // Generate types for this component
        // Input type
        let input_type = TilStreamType::new(
            &format!("{}InStream", component_name),
            gen_params.bit_width,
            gen_params.epc,
            self.outer_nested + 1,
            Synchronicity::Sync,
            8,
        );

        interface.add_input_stream("input", input_type.clone());
        type_manager.register(input_type);

        // Output type
        let output_type = TilStreamType::new(
            &format!("{}OutStream", component_name),
            gen_params.bit_width,
            gen_params.epc,
            self.outer_nested + 2,
            Synchronicity::Sync,
            8,
        );

        interface.add_output_stream("output", output_type.clone());
        type_manager.register(output_type);

        interface
    }

    fn get_preffered_name(&self) -> String {
        "array_parser".to_string()
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

impl JsonComponentValue for Array {
    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("Array parser\nO: {}, I: {}", self.outer_nested, self.inner_nested)
        )
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        match &self.value {
            Some(child) => vec![*child.clone()],
            None => vec![],
        }
    }

    fn num_children(&self) -> usize {
        match &self.value {
            Some(_) => 1,
            None => 0,
        }
    }
}