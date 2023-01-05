use self::{components::JsonComponent, analyzer::Analyzer};

mod components;
mod visualization;
pub mod generator;
pub mod analyzer;
pub mod types;
// mod vhdl;
mod til;


pub struct Generator {
    root: Option<JsonComponent>,
    analyzer: Analyzer,
    gen_params: GeneratorParams,
}

#[derive(Default, Clone)]
pub struct GeneratorParams {
    epc: usize,
    bit_width: usize,
    int_width: usize,
    output_dir: String,
}