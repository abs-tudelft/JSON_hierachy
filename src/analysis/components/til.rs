use indoc::formatdoc;

use crate::analysis::{NameReg, GeneratorParams};

use super::{JsonComponent, JsonType};

/**********************************************************************************
 * Implementation of how a component is translated to TIL                        *
 **********************************************************************************/

impl JsonComponent {
    pub fn to_til(&self, name_reg: &mut NameReg, gen_params: &GeneratorParams) -> String {
        match self {
            JsonComponent::Value { data_type, outer_nested } => {
                let mut til = String::new();
                
                til.push_str(&
                    match data_type {
                        JsonType::String => {
                            let comp_name = name_reg.register("string_parser", *outer_nested);

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
                                outer_nested + 1,

                                comp_name,
                                gen_params.bit_width,
                                gen_params.epc,
                                outer_nested + 1,

                                comp_name,
                                comp_name,
                                comp_name,
                            )
                        },
                        JsonType::Integer => {
                            let comp_name = name_reg.register("int_parser", *outer_nested);

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
                                outer_nested + 1,

                                comp_name,
                                gen_params.int_width,
                                outer_nested,

                                comp_name,
                                comp_name,
                                comp_name,
                            )
                        },
                        JsonType::Boolean => {
                            let comp_name = name_reg.register("bool_parser", *outer_nested);

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
                                outer_nested + 1,

                                comp_name,
                                outer_nested,

                                comp_name,
                                comp_name,
                                comp_name,
                            )
                        }
                    }
                );

                // Add trailing new line
                til.push_str("\n");

                til
            },
            JsonComponent::Array { outer_nested, inner_nested, value } => {
                let mut til = String::new();

                let comp_name = name_reg.register("array_parser", *outer_nested);

                til.push_str(&formatdoc!(
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
                        outer_nested + 1,

                        comp_name,
                        gen_params.bit_width,
                        gen_params.epc,
                        outer_nested + 2,

                        comp_name,
                        comp_name,
                        comp_name,
                    )
                );

                // Add trailing new line
                til.push_str("\n");

                // Recursively generate TIL for the child component, if it exists
                if let Some(value) = value {
                    til.push_str(&value.to_til(name_reg, gen_params));
                }
                
                til
            },
            JsonComponent::Key { name, outer_nested, value } => {
                let mut til = String::new();

                let comp_name = name_reg.register(&format!("{}_key_filter", name), *outer_nested);

                // TODO: add in regex matcher
                til.push_str(&formatdoc!(
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
                        outer_nested + 1,

                        comp_name,
                        gen_params.bit_width,
                        gen_params.epc,
                        outer_nested + 1,

                        comp_name,
                        comp_name,
                        comp_name,
                    )
                );

                // Add trailing new line
                til.push_str("\n");
                
                // Recursively generate TIL for the value of the record, if it exists
                if let Some(value) = value {
                    til.push_str(&value.to_til(name_reg, gen_params));
                }

                til
            },
            JsonComponent::Object { outer_nested, inner_nested, records } => {
                let mut til = String::new();

                let comp_name = name_reg.register("record_parser", *outer_nested);

                til.push_str(&formatdoc!(
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
                        outer_nested + 1,

                        comp_name,
                        gen_params.bit_width,
                        gen_params.epc,
                        outer_nested + 2,

                        comp_name,
                        comp_name,
                        comp_name,
                    )
                );

                // Add trailing new line
                til.push_str("\n");

                // Recursively generate TIL for child components
                for child in records {
                    til.push_str(&child.to_til(name_reg, gen_params));
                }

                til
            },
        }
    }
}