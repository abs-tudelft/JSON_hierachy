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
    fn to_til_component(&self, gen_tools: &mut GenTools, gen_params: &GeneratorParams) -> (Option<String>, Option<String>) {
        let comp_name = gen_tools.name_map.register("key_filter", self.outer_nested);

        let mut til = String::new();

        // Type generation
        // Register the matcher type
        let type_exists = gen_tools.type_reg.register("MatcherStream");
        if !type_exists {
            til.push_str(
                &formatdoc!(
                    "
                    type MatcherStream = Stream (
                        data: Bits(1),
                        throughput: {},
                        dimensionality: 1,
                        synchronicity: Sync,
                        complexity: 8,
                    );\n
                ", gen_params.epc)
            );
        }

        // Register the key type
        // Keys cannot be registered yet due not being generic
        // let type_exists = gen_tools.type_reg.register(&format!("KeyStream");
        // if !type_exists {
        //     Here comes the key stream type
        // }
        //
        // Fall back for now:
        til.push_str(
            &formatdoc!(
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
                ", 
                comp_name, 
                gen_params.bit_width,
                gen_params.epc,
                self.outer_nested + 1,
    
                comp_name,
                gen_params.bit_width,
                gen_params.epc,
                self.outer_nested + 1
            )
        );

        // Component definition
        til.push_str(
            &formatdoc!(
                "
                streamlet {} = (
                    input: in {}InStream,
                    matcherIn: in MatcherStream,
                    matcherOut: out MatcherStream,
                    output: out {}OutStream,
                );
                ",
                comp_name,
                comp_name,
                comp_name,
            )
        );

        (Some(comp_name), Some(til))
    }

    fn to_til_signal(&self, component_name: &str, parent_name: &str) -> Option<String> {
        Some(
            formatdoc!(
                "
                {}.output -- {}.input ;
                ",
                parent_name,
                component_name,
            )
        )
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