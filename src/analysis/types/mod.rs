use std::{fmt::{Display, Formatter}, cmp::Ordering};

use indoc::formatdoc;

use super::gen_tools::type_manager::StreamType;

#[derive(Clone)]
pub struct TilComponent {
    name: String,
    streams: TilStreamingInterface,
    implementation: Option<TilImplementationType>,
}

impl TilComponent {
    pub fn new(name: &str) -> TilComponent {
        TilComponent {
            name: String::from(name),
            streams: TilStreamingInterface::new(),
            implementation: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_implementation(&mut self, implementation: TilImplementationType) {
        self.implementation = Some(implementation);
    }

    pub fn set_streaming_interface(&mut self, stream_interface: TilStreamingInterface) {
        self.streams = stream_interface;
    }

    pub fn get_streams(&self) -> &TilStreamingInterface {
        &self.streams
    }

    pub fn get_streams_mut(&mut self) -> &mut TilStreamingInterface {
        &mut self.streams
    }

    pub fn get_implementation(&self) -> &Option<TilImplementationType> {
        &self.implementation
    }
}

impl Display for TilComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut comp_def = String::new();
        let mut generic_defs = String::new();
        let mut stream_defs = String::new();

        if !self.get_streams().get_generics().is_empty() {
            for generic in self.get_streams().get_generics() {
                generic_defs.push_str(
                    &format!("{},\n", generic)
                );
            }
        }

        for stream in self.get_streams().get_streams() {
            stream_defs.push_str(
                &format!("{},\n", stream)
            );
        }

        comp_def.push_str(
            &formatdoc!(
                "
                streamlet {} = <
                    {}
                > (
                    {}
                )",
                self.get_name(),
                generic_defs,
                stream_defs
            )
        );

        write!(f, "{}", comp_def)
    }
}

#[derive(Clone)]
pub enum TilImplementationType {
    Path(String),
    Inline(TilInlineImplementation),
}

#[derive(Clone)]
pub struct TilInlineImplementation {
    instances: Vec<TilInstance>,
    signals: Vec<TilSignal>,
}

impl TilInlineImplementation {
    pub fn new() -> TilInlineImplementation {
        TilInlineImplementation {
            instances: Vec::new(),
            signals: Vec::new(),
        }
    }

    pub fn add_instance(&mut self, component_name: String) -> String{
        let instance_name = format!("{}_inst", component_name);
        self.instances.push(TilInstance::new(&component_name, &instance_name));
        instance_name
    }

    pub fn add_signal(&mut self, signal: TilSignal) {
        self.signals.push(signal);
    }

    pub fn add_multiple_signals(&mut self, signals: Vec<TilSignal>) {
        self.signals.extend(signals);
    }

    pub fn get_instances(&self) -> &Vec<TilInstance> {
        &self.instances
    }

    pub fn get_signals(&self) -> &Vec<TilSignal> {
        &self.signals
    }
}

#[derive(Clone)]
pub struct TilInstance {
    component_name: String,
    instance_name: String,
}

impl TilInstance {
    pub fn new(component_name: &str, instance_name: &str) -> TilInstance {
        TilInstance {
            component_name: String::from(component_name),
            instance_name: String::from(instance_name),
        }
    }

    pub fn to_til(&self) -> String {
        format!("{} = {};", self.instance_name, self.component_name)
    }
}

#[derive(Clone)]
pub struct TilSignal {
    source_inst_name: Option<String>,
    source_stream_name: String,
    dest_inst_name: Option<String>,
    dest_stream_name: String,
}

impl TilSignal {
    pub fn new(source_inst_name: &Option<String>, source_stream_name: &str, dest_inst_name: &Option<String>, dest_stream_name: &str) -> TilSignal {
        TilSignal {
            source_inst_name: source_inst_name.to_owned(),
            source_stream_name: String::from(source_stream_name),
            dest_inst_name: dest_inst_name.to_owned(),
            dest_stream_name: String::from(dest_stream_name),
        }
    }

    pub fn to_til(&self) -> String {
        let mut signal = String::new();

        // If the signal is from an instance, add the instance name
        if let Some(source_inst_name) = &self.source_inst_name {
            signal.push_str(&format!("{}.", source_inst_name));
        }

        // Add stream name
        signal.push_str(&self.source_stream_name);

        // Connector
        signal.push_str(" -- ");

        // If the signal is to an instance, add the instance name
        if let Some(dest_inst_name) = &self.dest_inst_name {
            signal.push_str(&format!("{}.", dest_inst_name));
        }
        
        // Add stream name
        signal.push_str(&format!("{};", self.dest_stream_name));
    
        signal
    }
}

#[derive(Clone)]
pub struct TilStreamingInterface {
    generics: Vec<Generic>,
    streams: Vec<TilStream>,
}

impl TilStreamingInterface {
    pub fn new() -> TilStreamingInterface {
        TilStreamingInterface {
            generics: Vec::new(),
            streams: Vec::new(),
        }
    }

    pub fn add_stream(&mut self, stream_name: &str, direction: TilStreamDirection, stream_type: StreamType) {
        self.streams.push(
            TilStream {
                name: String::from(stream_name),
                direction,
                stream_type,
            }
        );
    }

    pub fn add_generic(&mut self, generic: Generic) {
        self.generics.push(generic);
    }

    pub fn get_generics(&self) -> &Vec<Generic> {
        &self.generics
    }

    pub fn get_streams(&self) -> &Vec<TilStream> {
        &self.streams
    }

    pub fn get_input_streams(&self) -> Vec<&TilStream> {
        self.streams.iter().filter(|stream| {
            match stream.direction {
                TilStreamDirection::Input => true,
                TilStreamDirection::Output => false,
            }
        }).collect()
    }

    pub fn get_output_streams(&self) -> Vec<&TilStream> {
        self.streams.iter().filter(|stream| {
            match stream.direction {
                TilStreamDirection::Input => false,
                TilStreamDirection::Output => true,
            }
        }).collect()
    }
}

#[derive(Clone)]
pub struct TilStream {
    name: String,
    direction: TilStreamDirection,
    stream_type: StreamType,
}

impl TilStream {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &StreamType {
        &self.stream_type
    }
}

impl Display for TilStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} {}", self.get_name(), self.direction, self.get_type().to_instance_string())
    }
}

#[derive(Clone)]
pub enum TilStreamDirection {
    Input,
    Output,
}

impl Display for TilStreamDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TilStreamDirection::Input => write!(f, "in"),
            TilStreamDirection::Output => write!(f, "out"),
        }
    }
}

#[derive(Clone)]
pub struct StreamDim {
    name: Option<String>,
    additive: isize,
    value: usize
}

impl StreamDim {
    pub fn new(name: Option<String>, value: usize, additive: isize) -> StreamDim {
        StreamDim {
            name,
            additive,
            value,
        }
    }

    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }

    pub fn get_additive(&self) -> isize {
        self.additive
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn get_true_value(&self) -> usize {
        (self.value as isize + self.additive) as usize
    }
}

impl Display for StreamDim {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dim = String::new();

        let mut has_name = false;

        dim.push('<');

        if let Some(name) = &self.name {
            dim.push_str(&name.to_string());
            has_name = true;
        }

        if has_name {
            match self.additive.cmp(&0) {
                Ordering::Greater => dim.push_str(&format!("+{}", self.additive)),
                Ordering::Less => dim.push_str(&format!("{}", self.additive)),
                Ordering::Equal => {}
            }
        } else {
            dim.push_str(&format!("{}", self.get_true_value()));
        }        

        dim.push('>');

        write!(f, "{}", dim)
    }
}

#[derive(Clone)]
pub struct Generic {
    name: String,
    generic_type: GenericType,
}

impl Generic {
    pub fn new(name: &str, generic_type: GenericType) -> Generic {
        Generic {
            name: String::from(name),
            generic_type,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &GenericType {
        &self.generic_type
    }
}

impl Display for Generic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.get_name(), self.get_type())
    }
}

#[derive(Clone)]
pub enum GenericType {
    Integer(isize),
    Natural(usize),
    Positive(usize),
    Dimensionality(usize),
}

impl Display for GenericType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericType::Integer(value) => write!(f, "integer = {}", value),
            GenericType::Natural(value) => write!(f, "natural = {}", value),
            GenericType::Positive(value) => write!(f, "positive = {}", value),
            GenericType::Dimensionality(value) => write!(f, "dimensionality = {}", value),
        }
    }
}