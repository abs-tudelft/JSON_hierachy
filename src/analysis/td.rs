use super::{Generator, td};

/**********************************************************************************
 * Set of functions to generate TD code from the analyzed definitions            *
 **********************************************************************************/

 fn generate_namespace_def(namespace: &str) -> String {
    let mut prelude = String::new();
    prelude.push_str(&format!("package {};\n", namespace));

    prelude
}

fn generate_close_namespace() -> String {
    let mut postlude = String::new();
    postlude.push_str("\n");

    postlude
}

impl Generator {
    pub fn generate_td(&mut self) -> String {
        let mut td = String::new();

        td.push_str(&td::generate_namespace_def(&self.gen_params.namespace));

        let (type_defs, stream_defs) = self.analyzer.get_definitions();

        for type_def in type_defs {
            td.push_str(&type_def.get_td_type_def_string(&self.gen_params));
        }

        for stream_def in stream_defs {
            td.push_str(&format!("{}\n\n", stream_def.td()));
        }

        let top_component = self.analyzer.assemble_top_component().unwrap();
        td.push_str(&top_component.td());

        td.push_str(&td::generate_close_namespace());

        td
    }
}