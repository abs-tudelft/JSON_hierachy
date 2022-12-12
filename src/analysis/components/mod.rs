use json::JsonValue;
use self::analysis::analyze_element;

mod vhdl;
mod display;
mod analysis;

#[derive(Debug)]
pub enum JsonType {
    String,
    Integer,
    Boolean,
}

pub enum JsonComponent {
    Value {
        data_type: JsonType, 
        outer_nested: u16
    },
    Array{
        outer_nested: u16,
        inner_nested: u16,
        value: Option<Box<JsonComponent>>
    },
    Object {
        outer_nested: u16,
        inner_nested: u16,
        records: Vec<JsonComponent>
    },
    Key {
        name: String,
        outer_nested: u16,
        value: Option<Box<JsonComponent>>
    },
}

// Start the analysis of the parsed JSON
// Returns the root component
pub fn analyze(root: &JsonValue) -> Option<JsonComponent> {
    let (root_component, _) = analyze_element(root, 0, 0);
    root_component
}