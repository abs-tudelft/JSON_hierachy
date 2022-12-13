use indoc::indoc;

/**********************************************************************************
 * Set of functions to generate VHDL code around the components                   *
 **********************************************************************************/

pub fn generate_prelude() -> String {
    let mut prelude = String::new();
    prelude.push_str("namespace schemaparser {\n\t");

    prelude
}

pub fn generate_postlude() -> String {
    let mut postlude = String::new();
    postlude.push_str("}");

    postlude
}