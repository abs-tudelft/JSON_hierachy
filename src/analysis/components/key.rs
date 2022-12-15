use indoc::formatdoc;

use crate::analysis::{GeneratorParams, gen_tools::GenTools};

use super::{Key, Generatable, JsonComponent, Matcher};

impl Key {
    pub fn new(matcher: Matcher, outer_nested: u16, value: Option<Box<JsonComponent>>) -> Key {
        Key {
            matcher,
            outer_nested,
            value
        }
    }
}

impl Generatable for Key {
    fn to_til(&self, gen_tools: &mut GenTools, gen_params: &GeneratorParams) -> (Option<String>, Option<String>) {
        let comp_name = gen_tools.name_map.register("key_filter", self.outer_nested);

        let til = formatdoc!(
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
        );

        (Some(comp_name), Some(til))
    }

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
}