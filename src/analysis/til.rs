use super::{Generator, til};

/**********************************************************************************
 * Set of functions to generate TIL code from the analyzed definitions            *
 **********************************************************************************/

 fn generate_namespace_def(namespace: &str) -> String {
    let mut prelude = String::new();
    prelude.push_str(&format!("namespace {} {{\n    ", namespace));

    prelude
}

fn generate_close_namespace() -> String {
    let mut postlude = String::new();
    postlude.push_str("\n}");

    postlude
}

impl Generator {
    pub fn generate_til(&mut self) -> String {
        let mut til = String::new();

        til.push_str(&til::generate_namespace_def(&self.gen_params.namespace));

        let (type_defs, stream_defs) = self.analyzer.get_definitions();

        for type_def in type_defs {
            til.push_str(&type_def.get_type_def_string(&self.gen_params));
        }

        for stream_def in stream_defs {
            til.push_str(&format!("{}\n\n", stream_def));
        }

        let top_component = self.analyzer.assemble_top_component().unwrap();
        til.push_str(&top_component.to_string());

        til.push_str(&til::generate_close_namespace());

        til
    }
}