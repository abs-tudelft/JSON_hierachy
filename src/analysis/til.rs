use indoc::formatdoc;

use super::{Generator, components::{JsonComponent, Generatable}, til, GeneratorParams, gen_tools::GenTools};

/**********************************************************************************
 * Set of functions to generate VHDL code around the components                   *
 **********************************************************************************/

 fn generate_prelude() -> String {
    let mut prelude = String::new();
    prelude.push_str("namespace schemaparser {\n\t");

    prelude
}

fn generate_top_component(interconnects: &String, gen_params: &GeneratorParams) -> String {
    formatdoc!(
        "
        type TopInStream = Stream (
            data: Bits({}),
            throughput: {},
            dimensionality: 2,
            synchronicity: Sync,
            complexity: 8,
        );

        type TopOutStream = Stream (
            data: Bits({}),
            throughput: 1,
            dimensionality: 2,
            synchronicity: Sync,
            complexity: 8,
        );

        streamlet Top = (
            input: TopInStream,
            output: TopOutStream,
        ) {{
            impl {{
                {}
            }}
        }};
        ", 
        gen_params.bit_width,
        gen_params.epc,
        gen_params.int_width,
        interconnects
    )
}

fn generate_postlude() -> String {
    let mut postlude = String::new();
    postlude.push('}');

    postlude
}

/**********************************************************************************
 * Implementation of how a component is translated to TIL                        *
 **********************************************************************************/

impl Generator {
    pub fn generate_til(&mut self) -> String {
        match &self.root {
            Some(root) => {
                let mut til = String::new();

                til.push_str(&til::generate_prelude());

                let (comp_til, conn_til) = generate_component(root, &None, &mut self.gen_tools, &self.gen_params);

                til.push_str(&comp_til);
                til.push_str(&til::generate_top_component(&conn_til, &self.gen_params));

                til.push_str(&til::generate_postlude());

                til
            },
            None => String::from(""),
        }
    }

    
}

/// Generate TIL for a component and its children
/// 
/// # Arguments
/// The component to generate TIL for
/// The name of the parent component
/// The tools to use for generating TIL
/// The parameters to use for generating TIL
/// 
/// # Returns
/// A tuple of the component definition and connection definitions in TIL
fn generate_component(component: &JsonComponent, parent_name: &Option<String>, gen_tools: &mut GenTools, gen_params: &GeneratorParams) -> (String, String) {
    let mut comp_til = String::new();
    let mut conn_til = String::new();

    // Generate TIL for the current component
    let comp_def = component.to_til_component(gen_tools, gen_params); 

    let comp_name: Option<String>;

    match comp_def {
        (Some(comp_name_val), Some(comp_def_til)) => {
            comp_name = Some(comp_name_val.clone());

            comp_til.push_str(&comp_def_til);
            comp_til.push('\n');

            // Generate TIL for the connections of the current component
            if let Some(parent_name) = parent_name {
                let conn_def = component.to_til_signal(&comp_name_val, parent_name);

                // If there is a connection definition, add it to the connection TIL
                if let Some(conn_def) = conn_def {
                    conn_til.push_str(&conn_def);
                    conn_til.push('\n');
                }
            }
        },
        //If the component has no name, pass through the name of the parent
        (_, _) => {
            comp_name = parent_name.clone();
        },
    }

    // Recursively generate TIL for child components
    for ref child in component.get_children() {
        let (child_comp_def, child_conn_def) = generate_component(child, &comp_name, gen_tools, gen_params);
        comp_til.push_str(&child_comp_def);
        conn_til.push_str(&child_conn_def);
    }

    (comp_til, conn_til)
}