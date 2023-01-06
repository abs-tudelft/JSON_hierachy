use super::{Generator, til};

/**********************************************************************************
 * Set of functions to generate VHDL code around the components                   *
 **********************************************************************************/

 fn generate_prelude(namespace: &str) -> String {
    let mut prelude = String::new();
    prelude.push_str(&format!("namespace {} {{\n\t", namespace));

    prelude
}

fn generate_postlude() -> String {
    let mut postlude = String::new();
    postlude.push_str("\n}");

    postlude
}

/**********************************************************************************
 * Implementation of how a component is translated to TIL                        *
 **********************************************************************************/

impl Generator {
    pub fn generate_til(&mut self) -> String {
        let mut til = String::new();

        til.push_str(&til::generate_prelude(&self.gen_params.namespace));

        let (type_defs, stream_defs) = self.analyzer.get_definitions();

        for type_def in type_defs {
            til.push_str(&type_def.get_type_def_string(&self.gen_params));
        }

        for stream_def in stream_defs {
            til.push_str(&format!("{}\n\n", stream_def));
        }

        let top_component = self.analyzer.assemble_top_component().unwrap();
        til.push_str(&top_component.to_string());

        til.push_str(&til::generate_postlude());

        til
    }
}