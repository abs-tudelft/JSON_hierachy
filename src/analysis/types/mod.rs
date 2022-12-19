#[derive(Clone)]
pub struct TilComponent {
    name: String,
    pub streams: TilStreamingInterface,
    pub implementation: Option<TilImplementationType>,
}

impl TilComponent {
    pub fn new(name: &str) -> TilComponent {
        TilComponent {
            name: String::from(name),
            streams: TilStreamingInterface::new(),
            implementation: None,
        }
    }

    pub fn set_implementation(&mut self, implementation: TilImplementationType) {
        self.implementation = Some(implementation);
    }

    pub fn set_streaming_interface(&mut self, stream_interface: TilStreamingInterface) {
        self.streams = stream_interface;
    }
}

#[derive(Clone)]
pub enum TilImplementationType {
    Path(String),
    Inline(TilInlineImplementation),
}

#[derive(Clone)]
pub struct TilInlineImplementation {
    instances: Vec<TilComponent>,
    signals: Vec<TilSignal>,
}

impl TilInlineImplementation {
    pub fn new() -> TilInlineImplementation {
        TilInlineImplementation {
            instances: Vec::new(),
            signals: Vec::new(),
        }
    }

    pub fn add_instance(&mut self, instance: TilComponent) {
        self.instances.push(instance);
    }

    pub fn add_signal(&mut self, signal: TilSignal) {
        self.signals.push(signal);
    }
}

#[derive(Clone)]
pub struct TilSignal {
    sender_name: String,
    receiver_name: String,
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

    pub fn add_input_stream(&mut self, stream_name: &str, stream_type: TilStreamType) {
        self.input.push(
            TilStream {
                name: String::from(stream_name),
                stream_type
            }
        );
    }

    pub fn add_output_stream(&mut self, stream_name: &str, stream_type: TilStreamType) {
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
    stream_type: TilStreamType,
}

impl TilStream {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &TilStreamType {
        &self.stream_type
    }
}

#[derive(Clone)]
pub struct TilStreamType {
    pub name: String,
    pub data_bits: usize,
    pub throughput: usize,
    pub dimensionality: usize,
    pub synchronicity: Synchronicity,
    pub complexity: u8,
}

impl TilStreamType {
    pub fn new(type_name: &str, data_bits: usize, throughput: usize, dimensionality: usize, synchronicity: Synchronicity, complexity: u8) -> TilStreamType {
        TilStreamType {
            name: String::from(type_name),
            data_bits,
            throughput,
            dimensionality,
            synchronicity,
            complexity,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub enum Synchronicity {
    Sync,
    Flatten,
    Desync,
    FlatDesync,
}