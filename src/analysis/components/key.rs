use indoc::formatdoc;

use crate::analysis::{NameReg, GeneratorParams};

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
    fn to_til(&self, name_reg: &mut NameReg, gen_params: &GeneratorParams) -> String {
        let comp_name = name_reg.register(&format!("{}_key_filter", self.name), self.outer_nested);

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