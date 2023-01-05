use std::{fmt::{Formatter, Display}, cmp::Ordering};

use crate::analysis::analyzer::type_manager::StreamType;

#[derive(Clone)]
pub struct StreamTypeDecl {
    stream_type: StreamType,
    stream_dim: Option<StreamDim>,
}

impl StreamTypeDecl {
    pub fn new(stream_type: StreamType, stream_dim: Option<StreamDim>) -> StreamTypeDecl {
        StreamTypeDecl {
            stream_type,
            stream_dim,
        }
    }

    pub fn get_name(&self) -> &str {
        self.stream_type.get_name()
    }

    pub fn get_stream_dim(&self) -> &Option<StreamDim> {
        &self.stream_dim
    }
}

impl Display for StreamTypeDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut inst_str = String::new();
        inst_str.push_str(self.get_name());

        if let Some(dim) = &self.stream_dim {
            inst_str.push_str(&dim.to_string());
        }

        write!(f, "{}", inst_str)
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