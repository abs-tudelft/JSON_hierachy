use super::{Generator, components::{JsonComponent, Generatable}, til, NameReg, GeneratorParams};

/**********************************************************************************
 * Set of functions to generate VHDL code around the components                   *
 **********************************************************************************/

fn generate_prelude() -> String {
    let mut prelude = String::new();
    prelude.push_str("namespace schemaparser {\n\t");

    prelude
}

fn generate_postlude() -> String {
    let mut postlude = String::new();
    postlude.push_str("}");

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

                til.push_str(&generate_component(root, &mut self.name_map, &self.gen_params));

                til.push_str(&til::generate_postlude());

                til
            },
            None => String::from(""),
        }
    }

    
}

fn generate_component(component: &JsonComponent, name_reg: &mut NameReg, gen_params: &GeneratorParams) -> String {
    let mut til = String::new();

    til.push_str(&component.to_til(name_reg, gen_params));

    til.push_str("\n");

    // Recursively generate TIL for child components
    for ref child in component.get_children() {
        til.push_str(&generate_component(child, name_reg, gen_params));
    }

    til
}