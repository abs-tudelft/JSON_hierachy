use indoc::formatdoc;

use crate::analysis::{GenTools, GeneratorParams};

use super::{Array, JsonComponent, Generatable};

impl Array {
    pub fn new(outer_nested: u16, inner_nested: u16, value: Option<Box<JsonComponent>>) -> Array {
        Array {
            outer_nested,
            inner_nested,
            value,
        }
    }
}

impl Generatable for Array {
    fn to_til(&self, gen_tools: &mut GenTools, gen_params: &GeneratorParams) -> (Option<String>, Option<String>) {
        let comp_name = gen_tools.name_map.register("array_parser", self.outer_nested);        

        // Generate til for this component
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
            self.outer_nested + 2,

            comp_name,
            comp_name,
            comp_name,
        );

        (Some(comp_name), Some(til))
    }

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
}