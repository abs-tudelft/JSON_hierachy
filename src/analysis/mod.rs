use self::{components::JsonComponent, gen_tools::GenTools};

mod components;
mod visualization;
pub mod generator;
pub mod analyzer;
pub mod gen_tools;
// mod vhdl;
mod til;


pub struct Generator {
    root: Option<JsonComponent>,
    gen_tools: GenTools,
    gen_params: GeneratorParams,
}

pub struct GeneratorParams {
    epc: u16,
    bit_width: u16,
    int_width: u16,
    output_dir: String,
}