#[derive(Debug, Clone)]
pub enum JsonType {
    String,
    Integer,
    Boolean,
}

use super::{types::{TilStreamingInterface, TilSignal}, gen_tools::{type_manager::StreamType}};

pub trait Generatable {
    

    /// Generates the TIL for the component
    /// 
    /// Returns a tuple of (component_name, til_streamlet_definition)
    
    // fn get_input_type_params(&self, gen_params: &GeneratorParams) -> StreamType;
    // fn get_output_type_params(&self, gen_params: &GeneratorParams) -> StreamType;
    fn get_streaming_interface(&self) -> TilStreamingInterface;
    fn get_streaming_types(&self) -> Vec<StreamType>;
    fn get_signals(&self, instance_name: &Option<String>, instance_stream_name: &str, parent_name: &Option<String>, parent_stream_name: &str) -> Vec<TilSignal>;
    fn num_outgoing_signals(&self) -> usize;
    fn get_preffered_name(&self) -> String;
    fn get_nesting_level(&self) -> usize;
}

pub trait JsonComponentValue {
    fn get_children(&self) -> Vec<JsonComponent>;
    fn num_children(&self) -> usize;

    fn to_graph_node(&self) -> Option<String>;
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
    outer_nested: usize
}

mod array;
#[derive(Clone)]
pub struct Array {
    outer_nested: usize,
    inner_nested: usize,
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
    outer_nested: usize,
    inner_nested: usize,
    key: Key
}

mod key;
#[derive(Clone)]
pub struct Key {
    matcher: Matcher,
    outer_nested: usize,
    value: Option<Box<JsonComponent>>
}

mod matcher;
#[derive(Clone)]
pub struct Matcher {
    matcher: String,
    outer_nested: usize
}

impl JsonComponent {
    pub fn get_if_generatable(&self) -> Option<&dyn Generatable> {
        match self {
            JsonComponent::Value(value) => Some(value),
            JsonComponent::Array(array) => Some(array),
            JsonComponent::Object(_) => None,
            JsonComponent::Record(record) => Some(record),
            JsonComponent::Key(key) => Some(key),
            JsonComponent::Matcher(matcher) => Some(matcher)
        }
    }
}

// Pass through methods
impl JsonComponentValue for JsonComponent {
    fn get_children(&self) -> Vec<JsonComponent> {
        match self {
            JsonComponent::Value(value) => value.get_children(),
            JsonComponent::Array(array) => array.get_children(),
            JsonComponent::Object(object) => object.get_children(),
            JsonComponent::Record(record) => record.get_children(),
            JsonComponent::Key(key) => key.get_children(),
            JsonComponent::Matcher(matcher) => matcher.get_children()
        }
    }

    fn num_children(&self) -> usize {
        match self {
            JsonComponent::Value(value) => value.num_children(),
            JsonComponent::Array(array) => array.num_children(),
            JsonComponent::Object(object) => object.num_children(),
            JsonComponent::Record(record) => record.num_children(),
            JsonComponent::Key(key) => key.num_children(),
            JsonComponent::Matcher(matcher) => matcher.num_children()
        }
    }

    fn to_graph_node(&self) -> Option<String> {
        match self {
            JsonComponent::Value(value) => value.to_graph_node(),
            JsonComponent::Array(array) => array.to_graph_node(),
            JsonComponent::Object(object) => object.to_graph_node(),
            JsonComponent::Record(record) => record.to_graph_node(),
            JsonComponent::Key(key) => key.to_graph_node(),
            JsonComponent::Matcher(matcher) => matcher.to_graph_node()
        }
    }
}