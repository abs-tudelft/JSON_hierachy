use indoc::formatdoc;

use crate::analysis::{GenTools, GeneratorParams};

use super::{Record, JsonComponent, Generatable, Key};

impl Record {
    pub fn new(outer_nested: u16, inner_nested: u16, key: Key) -> Record {
        Record {
            outer_nested,
            inner_nested,
            key
        }
    }
}

impl Generatable for Record {
    fn to_til(&self, gen_tools: &mut GenTools, gen_params: &GeneratorParams) -> String {
        let comp_name = gen_tools.name_map.register("record_parser", self.outer_nested);

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
            self.outer_nested + 2,

            comp_name,
            comp_name,
            comp_name,
        )
    }

    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("Record parser\nO: {}, I: {}", self.outer_nested, self.inner_nested)
        )
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        vec![JsonComponent::Key(self.key.clone())]
    }
}