#[derive(Debug, Clone)]
pub enum JsonType {
    String,
    Integer,
    Boolean,
}

use super::{types::{TilStreamingInterface, TilSignal, TilStreamlet, til_streamlet::TilImplementationType}, GeneratorParams, analyzer::{type_manager::StreamType, file_manager::TemplateType}};

pub trait Generatable {
    

    /// Generates the TIL for the component
    /// 
    /// Returns a tuple of (component_name, til_streamlet_definition)
    
    // fn get_input_type_params(&self, gen_params: &GeneratorParams) -> StreamType;
    // fn get_output_type_params(&self, gen_params: &GeneratorParams) -> StreamType;
    fn get_streaming_interface(&self, gen_params: &GeneratorParams) -> TilStreamingInterface;
    fn get_streaming_types(&self) -> Vec<StreamType>;
    fn get_outgoing_signals(&self) -> Vec<TilSignal>;
    fn num_outgoing_signals(&self) -> usize;
    fn get_nesting_level(&self) -> usize;
    fn get_name(&self) -> &str;

    fn to_til_streamlet(&self, gen_params: &GeneratorParams) -> TilStreamlet {
        // Create a new component
        let mut entity = TilStreamlet::new(self.get_name());

        // Generate streaming interface
        let streaming_interface = self.get_streaming_interface(gen_params);

        // Add streaming interface to entity
        entity.set_streaming_interface(streaming_interface);

        // Set implementation path
        entity.set_implementation(TilImplementationType::Path("./vhdl_dir".to_string()));

        entity
    }

    fn get_instance_name(&self) -> String {
        format!("{}_inst", self.get_name())
    }

    fn get_file_type(&self) -> TemplateType;
}

pub trait JsonComponentValue {
    fn get_children(&self) -> Vec<JsonComponent>;
    fn num_children(&self) -> usize;

    fn to_graph_node(&self) -> String;
}

#[derive(Clone)]
pub enum JsonComponent {
    Value(Value),
    Array(Array),
    Record(Record),
    Key(Key),
    Matcher(Matcher),
}

mod value;
#[derive(Clone)]
pub struct Value {
    name: String,
    data_type: JsonType,
    outer_nested: usize
}

mod array;
#[derive(Clone)]
pub struct Array {
    name: String,
    outer_nested: usize,
    inner_nested: usize,
    value: Option<Box<JsonComponent>>
}

mod record;
#[derive(Clone)]
pub struct Record {
    name: String,
    outer_nested: usize,
    inner_nested: usize,
    keys: Vec<Key>
}

mod key;
#[derive(Clone)]
pub struct Key {
    name: String,
    matcher: Matcher,
    outer_nested: usize,
    value: Option<Box<JsonComponent>>
}

mod matcher;
#[derive(Clone)]
pub struct Matcher {
    name: String,
    holder_name: String,
    matcher: String,
    outer_nested: usize
}

impl JsonComponent {
    pub fn get_generatable(&self) -> &dyn Generatable {
        match self {
            JsonComponent::Value(value) => value,
            JsonComponent::Array(array) => array,
            JsonComponent::Record(record) => record,
            JsonComponent::Key(key) => key,
            JsonComponent::Matcher(matcher) => matcher
        }
    }
}

// Pass through methods
impl JsonComponentValue for JsonComponent {
    fn get_children(&self) -> Vec<JsonComponent> {
        match self {
            JsonComponent::Value(value) => value.get_children(),
            JsonComponent::Array(array) => array.get_children(),
            JsonComponent::Record(record) => record.get_children(),
            JsonComponent::Key(key) => key.get_children(),
            JsonComponent::Matcher(matcher) => matcher.get_children()
        }
    }

    fn num_children(&self) -> usize {
        match self {
            JsonComponent::Value(value) => value.num_children(),
            JsonComponent::Array(array) => array.num_children(),
            JsonComponent::Record(record) => record.num_children(),
            JsonComponent::Key(key) => key.num_children(),
            JsonComponent::Matcher(matcher) => matcher.num_children()
        }
    }

    fn to_graph_node(&self) -> String {
        match self {
            JsonComponent::Value(value) => value.to_graph_node(),
            JsonComponent::Array(array) => array.to_graph_node(),
            JsonComponent::Record(record) => record.to_graph_node(),
            JsonComponent::Key(key) => key.to_graph_node(),
            JsonComponent::Matcher(matcher) => matcher.to_graph_node()
        }
    }
}

impl From<Box<JsonComponent>> for Box<dyn Generatable> {
    fn from(value: Box<JsonComponent>) -> Self {
        match *value {
            JsonComponent::Value(value) => Box::new(value),
            JsonComponent::Array(array) => Box::new(array),
            JsonComponent::Record(record) => Box::new(record),
            JsonComponent::Key(key) => Box::new(key),
            JsonComponent::Matcher(matcher) => Box::new(matcher)
        }
    }
}