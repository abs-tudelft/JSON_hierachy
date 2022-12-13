use std::collections::HashMap;

use self::components::JsonComponent;

mod components;
mod visualization;
pub mod generator;
pub mod analyzer;
pub mod name_reg;
// mod vhdl;
mod til;

pub struct Generator {
    root: Option<JsonComponent>,
    name_map: NameReg,
    gen_params: GeneratorParams,
}

pub struct NameReg {
    name_map: HashMap<String, u32>,
}

pub struct GeneratorParams {
    epc: u16,
    bit_width: u16,
    int_width: u16,
}