use self::{streaming_interface::{TilStream, Generic}, til_streamlet::TilImplementationType};

pub mod stream_types;
pub mod signals;
pub mod streaming_interface;
pub mod til_streamlet;

#[derive(Clone)]
pub struct TilStreamlet {
    name: String,
    streams: TilStreamingInterface,
    implementation: Option<TilImplementationType>,
}

#[derive(Clone)]
pub enum TilSignal {
    Input {
        source_stream_name: String,
        dest_inst_name: String,
        dest_stream_name: String,
    },
    Intermediate {
        source_inst_name: String,
        source_stream_name: String,
        dest_inst_name: String,
        dest_stream_name: String,
    },
    Output {
        source_inst_name: String,
        source_stream_name: String,
        dest_stream_name: String,
        output_stream: TilStream,
    },
}

#[derive(Clone, Default)]
pub struct TilStreamingInterface {
    generics: Vec<Generic>,
    streams: Vec<TilStream>,
}