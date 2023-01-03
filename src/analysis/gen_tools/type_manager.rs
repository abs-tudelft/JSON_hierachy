use indoc::writedoc;

use crate::analysis::{GeneratorParams, types::StreamDim};

use super::TypeManager;

impl TypeManager {
    pub fn new() -> TypeManager {
        TypeManager {
            type_list: Vec::new(),
        }
    }

    /// Register a new data type
    pub fn register(&mut self, stream_type: StreamType) {
        // Check if type already exists
        if !self.type_list.contains(&stream_type) {
            self.type_list.push(stream_type);
        }
    }

    pub fn generate_type_defs(&self, gen_params: &GeneratorParams) -> String {
        let mut type_defs = String::new();

        for stream_type in &self.type_list {
            let type_params = stream_type.get_type_params(gen_params);

            let dim_str = match type_params.dimensionality {
                Dimensionality::Fixed(_) => "".to_string(),
                Dimensionality::Generic => format!("<{}: dimensionality = 2>", Dimensionality::Generic),
            };

            type_defs.push_str(&format!("type {}{} = {};\n\n", stream_type.get_name(), dim_str, type_params));
        }

        type_defs
    }
}

#[derive(Clone)]
pub enum StreamType {
    Json(StreamDim),
    Int(StreamDim),
    Bool(StreamDim),
    Record(StreamDim),
    MatcherMatch,
    MatcherStr,
}

impl StreamType {
    pub fn get_name(&self) -> &str {
        match self {
            StreamType::Json{..} => "JSONStream",
            StreamType::Int{..} => "IntParserStream",
            StreamType::Bool{..} => "BoolParserStream",
            StreamType::Record{..} => "RecordParserStream",
            StreamType::MatcherMatch => "MatcherMatchStream",
            StreamType::MatcherStr => "MatcherStrStream",
        }
    }

    fn get_type_params(&self, gen_params: &GeneratorParams) -> StreamParams {
        match self {
            StreamType::Json{..} =>  StreamParams::new(gen_params.bit_width, gen_params.epc, Dimensionality::Generic, Synchronicity::Sync, 8),
            StreamType::Int{..} => StreamParams::new(gen_params.int_width, 1, Dimensionality::Generic, Synchronicity::Sync, 2),
            StreamType::Bool{..} => StreamParams::new(1, 1, Dimensionality::Generic, Synchronicity::Sync, 2),
            StreamType::Record{..} => StreamParams::new(gen_params.bit_width + 1, gen_params.epc, Dimensionality::Generic, Synchronicity::Sync, 8),
            StreamType::MatcherMatch => StreamParams::new(1, gen_params.epc, Dimensionality::Fixed(1), Synchronicity::Sync, 8),
            StreamType::MatcherStr => StreamParams::new(gen_params.bit_width, gen_params.epc, Dimensionality::Fixed(1), Synchronicity::Sync, 8),
        }
    }

    pub fn to_instance_string(&self) -> String {
        let mut inst_str = String::new();
        inst_str.push_str(&self.get_name().to_string());

        inst_str.push_str(
            &match self {
                StreamType::Json(stream_dim) => stream_dim.to_string(),
                StreamType::Int(stream_dim) => stream_dim.to_string(),
                StreamType::Bool(stream_dim) => stream_dim.to_string(),
                StreamType::Record(stream_dim) => stream_dim.to_string(),
                StreamType::MatcherMatch => "".to_string(),
                StreamType::MatcherStr => "".to_string(),
            }
        );

        inst_str
    }

    pub fn get_stream_dim(&self) -> Option<StreamDim> {
        match self {
            StreamType::Json(stream_dim) => Some(stream_dim.clone()),
            StreamType::Int(stream_dim) => Some(stream_dim.clone()),
            StreamType::Bool(stream_dim) => Some(stream_dim.clone()),
            StreamType::Record(stream_dim) => Some(stream_dim.clone()),
            StreamType::MatcherMatch => None,
            StreamType::MatcherStr => None,
        }
    }
}

impl PartialEq for StreamType {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), 
            (StreamType::Json{..}, StreamType::Json{..}) | 
            (StreamType::Int{..}, StreamType::Int{..}) | 
            (StreamType::Bool{..}, StreamType::Bool{..}) | 
            (StreamType::Record{..}, StreamType::Record{..}) | 
            (StreamType::MatcherMatch, StreamType::MatcherMatch) | 
            (StreamType::MatcherStr, StreamType::MatcherStr)
        )
    }
}

struct StreamParams {
    pub data_bits: usize,
    pub throughput: usize,
    pub dimensionality: Dimensionality,
    pub synchronicity: Synchronicity,
    pub complexity: u8,
}

impl StreamParams {
    fn new(data_bits: usize, throughput: usize, dimensionality: Dimensionality, synchronicity: Synchronicity, complexity: u8) -> Self {
        StreamParams {
            data_bits,
            throughput,
            dimensionality,
            synchronicity,
            complexity,
        }
    }
}

impl std::fmt::Display for StreamParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writedoc!(
            f,
            "Stream (
                data: Bits({}),
                throughput: {},
                dimensionality: {},
                synchronicity: {:?},
                complexity: {},
            )",
            self.data_bits,
            self.throughput,
            self.dimensionality,
            self.synchronicity,
            self.complexity,
        )
    }
}

enum Dimensionality {
    Fixed(usize),
    Generic
}

impl std::fmt::Display for Dimensionality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dimensionality::Fixed(d) => write!(f, "{}", d),
            Dimensionality::Generic => write!(f, "d"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Synchronicity {
    Sync,
    Flatten,
    Desync,
    FlatDesync,
}