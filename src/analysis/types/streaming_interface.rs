use std::fmt::{Display, Formatter};

use super::{TilStreamingInterface, stream_types::StreamTypeDecl};

impl TilStreamingInterface {
    pub fn add_stream(&mut self, stream_name: &str, direction: TilStreamDirection, stream_type: StreamTypeDecl) {
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
    stream_type: StreamTypeDecl,
}

impl TilStream {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &StreamTypeDecl {
        &self.stream_type
    }
}

impl Display for TilStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} {}", self.get_name(), self.direction, self.get_type())
    }
}

#[derive(Clone, Copy)]
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


#[derive(Clone, Copy)]
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