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

fn generate_top_component(instances: String, interconnects: String, gen_params: &GeneratorParams) -> String {
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
            input: in TopInStream,
            output: out TopOutStream,
        ) {{
            impl: {{
                {}
                {}
            }}
        }};
        ", 
        gen_params.bit_width,
        gen_params.epc,
        gen_params.int_width,
        instances,
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

                let comp_til = generate_component(root, &None, &mut self.gen_tools, &self.gen_params);

                til.push_str(&comp_til.definition);
                til.push_str(&til::generate_top_component(comp_til.instances, comp_til.connections, &self.gen_params));

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
/// The TIL for the component and its children
fn generate_component(component: &JsonComponent, parent_inst_name: &Option<String>, gen_tools: &mut GenTools, gen_params: &GeneratorParams) -> Til {
    let mut til = Til::new();

    // Generate TIL for the current component
    let comp_def = component.to_til_component(gen_tools, gen_params); 

    let comp_inst_name: Option<String>;

    match comp_def {
        (Some(comp_name_val), Some(comp_def_til)) => {
            // Add the component definition to the TIL
            til.add_definition(comp_def_til);

            // Add the component instance to the TIL
            let inst_name = til.add_instance(comp_name_val);

            // Set the name of the current component
            comp_inst_name = Some(inst_name.clone());

            // Generate TIL for the connections of the current component
            if let Some(parent_inst_name) = parent_inst_name {
                let conn_def = component.to_til_signal(&inst_name, parent_inst_name);

                // If there is a connection definition, add it to the connection TIL
                if let Some(conn_def) = conn_def {
                    til.add_connection(conn_def);
                }
            }
        },
        //If the component has no name, pass through the name of the parent
        (_, _) => {
            comp_inst_name = parent_inst_name.clone();
        },
    }

    // Recursively generate TIL for child components
    for ref child in component.get_children() {
        let child_til = generate_component(child, &comp_inst_name, gen_tools, gen_params);

        // Add the child component to the TIL
        til.add_til(child_til);
    }

    til
}

struct Til {
    definition: String,
    instances: String,
    connections: String,
}

impl Til {
    fn new() -> Self {
        Self {
            definition: String::new(),
            instances: String::new(),
            connections: String::new(),
        }
    }

    fn add_definition(&mut self, definition: String) {
        self.definition.push_str(&definition);
        self.definition.push('\n');
    }

    fn add_instance(&mut self, component_name: String) -> String {
        let instance_name = format!("{}_inst", component_name);

        self.instances.push_str(&format!("{} = {};", instance_name, component_name));
        self.instances.push('\n');

        instance_name
    }

    fn add_connection(&mut self, connection: String) {
        self.connections.push_str(&connection);
        self.connections.push('\n');
    }

    fn add_til(&mut self, til: Til) {
        self.definition.push_str(&til.definition);
        self.instances.push_str(&til.instances);
        self.connections.push_str(&til.connections);
    }
}