use self::components::JsonComponent;

mod components;
mod visualization;
pub mod generator;
mod vhdl;

pub struct Generator {
    root: Option<JsonComponent>,
}