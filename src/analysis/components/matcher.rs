use indoc::formatdoc;

use crate::analysis::{gen_tools::GenTools, GeneratorParams};

use super::{JsonComponent, Matcher, Generatable};

impl Matcher {
    pub fn new(matcher: String, outer_nested: u16) -> Matcher {
        Matcher {
            matcher,
            outer_nested
        }
    }
}

impl Generatable for Matcher {
    fn to_til(&self, gen_tools: &mut GenTools, gen_params: &GeneratorParams) -> (Option<String>, Option<String>) {
        let comp_name = gen_tools.name_map.register(&format!("{}_matcher", self.matcher), self.outer_nested);

        // Check if it is the first matcher
        let first = gen_tools.match_manager.is_empty();

        // Generate the matcher
        gen_tools.match_manager.add_matcher(&self.matcher, gen_params);

        let mut til = String::new();

        // If the matcher type already exists, don't generate it again
        if first {
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

        til.push_str(
            &formatdoc!(
                "
                streamlet {} = (
                    input: in MatcherStream,
                    output: out MatcherStream,
                );
                ", 
                comp_name
            )
        );

        (Some(comp_name), Some(til))
    }

    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("Regex matcher\n\"{}\"", self.matcher)
        )   
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        Vec::new()
    }
}