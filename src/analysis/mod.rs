use self::{components::JsonComponent, gen_tools::GenTools};

mod components;
mod visualization;
pub mod generator;
pub mod analyzer;
pub mod gen_tools;
pub mod types;
// mod vhdl;
mod til;


pub struct Generator {
    root: Option<JsonComponent>,
    gen_tools: GenTools,
    gen_params: GeneratorParams,
}

pub struct GeneratorParams {
    epc: usize,
    bit_width: usize,
    int_width: usize,
    output_dir: String,
}