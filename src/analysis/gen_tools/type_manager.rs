use enum_map::{Enum, EnumMap};
use indoc::writedoc;

use crate::analysis::GeneratorParams;

use super::TypeManager;

impl TypeManager {
    pub fn new() -> TypeManager {
        let map: EnumMap<StreamType, bool> = EnumMap::default();

        // Set all type usage to false
        map.map(|_, _| false);

        TypeManager {
            type_list: map,
        }
    }

    /// Register a new data type
    pub fn register(&mut self, stream_type: StreamType) {
        self.type_list[stream_type] = true;
    }

    pub fn generate_type_defs(&self, gen_params: &GeneratorParams) -> String {
        let mut type_defs = String::new();

        for (stream_type, type_exists) in &self.type_list {
            if *type_exists {
                let type_params = stream_type.get_type_params(gen_params);

                let dim_str = match type_params.dimensionality {
                    Dimensionality::Fixed(_) => "".to_string(),
                    Dimensionality::Generic => format!("<{}: dimensionality = 2>", Dimensionality::Generic),
                };

                type_defs.push_str(&format!("type {}{} = {};\n\n", stream_type.get_name(), dim_str, type_params));
            }
        };

        type_defs
    }
}

#[derive(Enum, Clone)]
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