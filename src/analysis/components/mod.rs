#[derive(Debug, Clone)]
pub enum JsonType {
    String,
    Integer,
    Boolean,
}

use crate::analysis::{GenTools, GeneratorParams};

pub trait Generatable {
    fn get_children(&self) -> Vec<JsonComponent>;

    fn to_graph_node(&self) -> Option<String>;

    /// Generates the TIL for the component
    /// 
    /// Returns a tuple of (component_name, til_streamlet_definition)
    fn to_til(&self, gen_tools: &mut GenTools, gen_params: &GeneratorParams) -> (Option<String>, Option<String>);
}

#[derive(Clone)]
pub enum JsonComponent {
    Value(Value),
    Array(Array),
    Object(Object),
    Record(Record),
    Key(Key),
    Matcher(Matcher),
}

mod value;
#[derive(Clone)]
pub struct Value {
    data_type: JsonType,
    outer_nested: u16
}

mod array;
#[derive(Clone)]
pub struct Array {
    outer_nested: u16,
    inner_nested: u16,
    value: Option<Box<JsonComponent>>
}

mod object;
#[derive(Clone)]
pub struct Object {
    records: Vec<Record>
}

mod record;
#[derive(Clone)]
pub struct Record {
    outer_nested: u16,
    inner_nested: u16,
    key: Key
}

mod key;
#[derive(Clone)]
pub struct Key {
    matcher: Matcher,
    outer_nested: u16,
    value: Option<Box<JsonComponent>>
}

mod matcher;
#[derive(Clone)]
pub struct Matcher {
    matcher: String,
    outer_nested: u16
}

// Pass through to the underlying implementation
impl Generatable for JsonComponent {
    fn get_children(&self) -> Vec<JsonComponent> {
        match self {
            JsonComponent::Value(value) => value.get_children(),
            JsonComponent::Array(array) => array.get_children(),
            JsonComponent::Object(object) => object.get_children(),
            JsonComponent::Record(record) => record.get_children(),
            JsonComponent::Key(key) => key.get_children(),
            JsonComponent::Matcher(matcher) => matcher.get_children(),
        }
    }

    fn to_graph_node(&self) -> Option<String> {
        match self {
            JsonComponent::Value(value) => value.to_graph_node(),
            JsonComponent::Array(array) => array.to_graph_node(),
            JsonComponent::Object(object) => object.to_graph_node(),
            JsonComponent::Record(record) => record.to_graph_node(),
            JsonComponent::Key(key) => key.to_graph_node(),
            JsonComponent::Matcher(matcher) => matcher.to_graph_node(),
        }
    }

    fn to_til(&self, gen_tools: &mut GenTools, gen_params: &GeneratorParams) -> (Option<String>, Option<String>) {
        match self {
            JsonComponent::Value(value) => value.to_til(gen_tools, gen_params),
            JsonComponent::Array(array) => array.to_til(gen_tools, gen_params),
            JsonComponent::Object(object) => object.to_til(gen_tools, gen_params),
            JsonComponent::Record(record) => record.to_til(gen_tools, gen_params),
            JsonComponent::Key(key) => key.to_til(gen_tools, gen_params),
            JsonComponent::Matcher(matcher) => matcher.to_til(gen_tools, gen_params),
        }
    }
}