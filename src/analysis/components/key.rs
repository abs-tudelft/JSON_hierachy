use indoc::formatdoc;

use crate::analysis::{GeneratorParams, gen_tools::GenTools};

use super::{Key, Generatable, JsonComponent};

impl Key {
    pub fn new(name: String, outer_nested: u16, value: Option<Box<JsonComponent>>) -> Key {
        Key {
            name,
            outer_nested,
            value
        }
    }
}

impl Generatable for Key {
    fn to_til(&self, gen_tools: &mut GenTools, gen_params: &GeneratorParams) -> String {
        let comp_name = gen_tools.name_map.register(&format!("{}_key_filter", self.name), self.outer_nested);

        // Generate a matcher
        gen_tools.match_manager.add_matcher(&self.name, gen_params);

        formatdoc!(
            "
            type {}InStream = Stream (
                data: Bits({}),
                throughput: {},
                dimensionality: {},
                synchronicity: Sync,
                complexity: 8,
            );

            type {}OutStream = Stream (
                data: Bits({}),
                throughput: {},
                dimensionality: {},
                synchronicity: Sync,
                complexity: 8,
            );

            streamlet {} = (
                input: in {}InStream,
                output: out {}OutStream,
            );
            ", 
            comp_name, 
            gen_params.bit_width,
            gen_params.epc,
            self.outer_nested + 1,

            comp_name,
            gen_params.bit_width,
            gen_params.epc,
            self.outer_nested + 1,

            comp_name,
            comp_name,
            comp_name,
        )
    }

    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("Key filter\nMatch: \"{}\"\nO: {}", self.name, self.outer_nested)
        )
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        match &self.value {
            Some(child) => vec![*child.clone()],
            None => vec![],
        }
    }
}