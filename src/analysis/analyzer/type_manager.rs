use std::fmt::{Display, Formatter};
use enumset::{EnumSetType, EnumSet};
use indoc::writedoc;

use crate::analysis::{GeneratorParams, components::Generatable};

pub(super) struct TypeManager {
    type_list: EnumSet<StreamType>,
}

impl TypeManager {
    pub fn new() -> TypeManager {
        TypeManager {
            type_list: EnumSet::new(),
        }
    }

    /// Register a new data type
    pub fn register(&mut self, stream_type: StreamType) {
        self.type_list.insert(stream_type);
    }

    pub fn register_from_component(&mut self, component: &dyn Generatable) {
        for stream_type in component.get_streaming_types() {
            self.register(stream_type);
        }
    }

    // Get stream type definitions
    pub fn get_stream_types(&self) -> Vec<StreamType> {
        self.type_list.iter().collect()
    }
}

impl Default for TypeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(EnumSetType)]
pub enum StreamType {
    Json,
    Int,
    Bool,
    Record,
    MatcherMatch,
    MatcherStr,
}

impl StreamType {
    pub fn get_name(&self) -> &str {
        match self {
            StreamType::Json => "JSONStream",
            StreamType::Int => "IntParserStream",
            StreamType::Bool => "BoolParserStream",
            StreamType::Record => "RecordParserStream",
            StreamType::MatcherMatch => "MatcherMatchStream",
            StreamType::MatcherStr => "MatcherStrStream",
        }
    }

    fn get_type_params(&self, gen_params: &GeneratorParams) -> StreamParams {
        match self {
            StreamType::Json =>  StreamParams::new(gen_params.bit_width, gen_params.epc, Dimensionality::Generic, Synchronicity::Sync, 8),
            StreamType::Int => StreamParams::new(gen_params.int_width, 1, Dimensionality::Generic, Synchronicity::Sync, 2),
            StreamType::Bool => StreamParams::new(1, 1, Dimensionality::Generic, Synchronicity::Sync, 2),
            StreamType::Record => StreamParams::new(gen_params.bit_width + 1, gen_params.epc, Dimensionality::Generic, Synchronicity::Sync, 8),
            StreamType::MatcherMatch => StreamParams::new(1, gen_params.epc, Dimensionality::Fixed(1), Synchronicity::Sync, 8),
            StreamType::MatcherStr => StreamParams::new(gen_params.bit_width, gen_params.epc, Dimensionality::Fixed(1), Synchronicity::Sync, 8),
        }
    }

    pub fn get_type_def_string(&self, gen_params: &GeneratorParams) -> String {
        let type_params = self.get_type_params(gen_params);

        let dim_str = match type_params.dimensionality {
            Dimensionality::Fixed(_) => "".to_string(),
            Dimensionality::Generic => format!("<{}: dimensionality = 2>", Dimensionality::Generic),
        };

        format!("type {}{} = {};\n\n", self.get_name(), dim_str, type_params)
    }

    pub fn get_td_type_def_string(&self, gen_params: &GeneratorParams) -> String {
        let type_params = self.get_type_params(gen_params);

        let dim_str = match type_params.dimensionality {
            Dimensionality::Fixed(_) => "".to_string(),
            Dimensionality::Generic => format!("<{}: dimensionality = 2>", Dimensionality::Generic),
        };

        format!("\n{} = {};\n", self.get_name(), type_params.td())
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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

    pub fn td(&self) -> String {
        format!(
"Stream(
    Bit({}),
    throughput = {}.0,
    dimension = {},
    synchronicity = \"{:?}\",
    complexity = {}
)",
        self.data_bits,
        self.throughput,
        self.dimensionality,
        self.synchronicity,
        self.complexity,)
    }
}

impl Display for StreamParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Synchronicity {
    Sync,
    Flatten,
    Desync,
    FlatDesync,
}


// #[derive(Clone)]
// pub enum StreamType {
//     Json(StreamDim),
//     Int(StreamDim),
//     Bool(StreamDim),
//     Record(StreamDim),
//     MatcherMatch,
//     MatcherStr,
// }

// impl StreamType {
//     pub fn get_name(&self) -> &str {
//         match self {
//             StreamType::Json{..} => "JSONStream",
//             StreamType::Int{..} => "IntParserStream",
//             StreamType::Bool{..} => "BoolParserStream",
//             StreamType::Record{..} => "RecordParserStream",
//             StreamType::MatcherMatch => "MatcherMatchStream",
//             StreamType::MatcherStr => "MatcherStrStream",
//         }
//     }

//     fn get_type_params(&self, gen_params: &GeneratorParams) -> StreamParams {
//         match self {
//             StreamType::Json{..} =>  StreamParams::new(gen_params.bit_width, gen_params.epc, Dimensionality::Generic, Synchronicity::Sync, 8),
//             StreamType::Int{..} => StreamParams::new(gen_params.int_width, 1, Dimensionality::Generic, Synchronicity::Sync, 2),
//             StreamType::Bool{..} => StreamParams::new(1, 1, Dimensionality::Generic, Synchronicity::Sync, 2),
//             StreamType::Record{..} => StreamParams::new(gen_params.bit_width + 1, gen_params.epc, Dimensionality::Generic, Synchronicity::Sync, 8),
//             StreamType::MatcherMatch => StreamParams::new(1, gen_params.epc, Dimensionality::Fixed(1), Synchronicity::Sync, 8),
//             StreamType::MatcherStr => StreamParams::new(gen_params.bit_width, gen_params.epc, Dimensionality::Fixed(1), Synchronicity::Sync, 8),
//         }
//     }

//     pub fn to_instance_string(&self) -> String {
//         let mut inst_str = String::new();
//         inst_str.push_str(self.get_name());

//         inst_str.push_str(
//             &match self {
//                 StreamType::Json(stream_dim) => stream_dim.to_string(),
//                 StreamType::Int(stream_dim) => stream_dim.to_string(),
//                 StreamType::Bool(stream_dim) => stream_dim.to_string(),
//                 StreamType::Record(stream_dim) => stream_dim.to_string(),
//                 StreamType::MatcherMatch => "".to_string(),
//                 StreamType::MatcherStr => "".to_string(),
//             }
//         );

//         inst_str
//     }
// }

// impl PartialEq for StreamType {
//     fn eq(&self, other: &Self) -> bool {
//         matches!((self, other), 
//             (StreamType::Json{..}, StreamType::Json{..}) | 
//             (StreamType::Int{..}, StreamType::Int{..}) | 
//             (StreamType::Bool{..}, StreamType::Bool{..}) | 
//             (StreamType::Record{..}, StreamType::Record{..}) | 
//             (StreamType::MatcherMatch, StreamType::MatcherMatch) | 
//             (StreamType::MatcherStr, StreamType::MatcherStr)
//         )
//     }
// }

// impl std::fmt::Display for StreamType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let type_params = self.get_type_params(gen_params);

//         let dim_str = match type_params.dimensionality {
//             Dimensionality::Fixed(_) => "".to_string(),
//             Dimensionality::Generic => format!("<{}: dimensionality = 2>", Dimensionality::Generic),
//         };
        
//         write!(f, "type {}{} = {};\n\n", self.get_name(), dim_str, type_params)
//     }
// }

