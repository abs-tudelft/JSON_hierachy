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
        self.instances.push(TilInstance::new(&component_name, &instance_name.clone()));
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
    input: Vec<TilStream>,
    output: Vec<TilStream>,
}

impl TilStreamingInterface {
    pub fn new() -> TilStreamingInterface {
        TilStreamingInterface {
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    pub fn add_input_stream(&mut self, stream_name: &str, stream_type: StreamType) {
        self.input.push(
            TilStream {
                name: String::from(stream_name),
                stream_type
            }
        );
    }

    pub fn add_output_stream(&mut self, stream_name: &str, stream_type: StreamType) {
        self.output.push(
            TilStream {
                name: String::from(stream_name),
                stream_type
            }
        );
    }

    pub fn get_input_streams(&self) -> &Vec<TilStream> {
        &self.input
    }

    pub fn get_output_streams(&self) -> &Vec<TilStream> {
        &self.output
    }
}

#[derive(Clone)]
pub struct TilStream {
    name: String,
    stream_type: StreamType,
}

impl TilStream {
    pub fn new(stream_name: &str, stream_type: StreamType) -> TilStream {
        TilStream {
            name: String::from(stream_name),
            stream_type
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &StreamType {
        &self.stream_type
    }
}

#[derive(Clone)]
pub struct TilStreamType {
    name: String,
    params: TilStreamParam
}

impl TilStreamType {
    pub fn new(type_name: &str, stream_params: TilStreamParam) -> TilStreamType {
        TilStreamType {
            name: String::from(type_name),
            params: stream_params
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_params(&self) -> &TilStreamParam {
        &self.params
    }
}

#[derive(Clone)]
pub struct TilStreamParam {
    pub data_bits: usize,
    pub throughput: usize,
    pub dimensionality: usize,
    pub synchronicity: Synchronicity,
    pub complexity: u8,
}

impl TilStreamParam {
    pub fn new(data_bits: usize, throughput: usize, dimensionality: usize, synchronicity: Synchronicity, complexity: u8) -> TilStreamParam {
        TilStreamParam {
            data_bits,
            throughput,
            dimensionality,
            synchronicity,
            complexity,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Synchronicity {
    Sync,
    Flatten,
    Desync,
    FlatDesync,
}