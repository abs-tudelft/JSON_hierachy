use indoc::formatdoc;

use crate::analysis::{GeneratorParams, gen_tools::GenTools};

use super::{JsonComponent, JsonType, Value, Generatable};

impl Value {
    pub fn new(data_type: JsonType, outer_nested: u16) -> Value {
        Value {
            data_type,
            outer_nested,
        }
    }
}

impl Generatable for Value {
    fn to_til(&self, gen_tools: &mut GenTools, gen_params: &GeneratorParams) -> String {
        match self.data_type {
            JsonType::String => {
                let comp_name = gen_tools.name_map.register("string_parser", self.outer_nested);

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
            },
            JsonType::Integer => {
                let comp_name = gen_tools.name_map.register("int_parser", self.outer_nested);

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
                        throughput: 1,
                        dimensionality: {},
                        synchronicity: Sync,
                        complexity: 2,
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
                    gen_params.int_width,
                    self.outer_nested,

                    comp_name,
                    comp_name,
                    comp_name,
                )
            },
            JsonType::Boolean => {
                let comp_name = gen_tools.name_map.register("bool_parser", self.outer_nested);

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
                        data: Bits(1),
                        throughput: 1,
                        dimensionality: {},
                        synchronicity: Sync,
                        complexity: 2,
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
                    self.outer_nested,

                    comp_name,
                    comp_name,
                    comp_name,
                )
            }
        }
    }

    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("{:?} parser\nO: {}", self.data_type, self.outer_nested)
        )
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        Vec::new()
    }
}