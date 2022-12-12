use self::components::JsonComponent;

mod components;
mod visualization;
pub mod generator;

pub struct Generator {
    root: Option<JsonComponent>,
}