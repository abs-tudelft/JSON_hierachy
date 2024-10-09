use std::fmt::{Display, Formatter};

use super::TilSignal;

impl Display for TilSignal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut signal = String::new();

        // If the signal is from an instance, add the instance name
        if let Some(source_inst_name) = self.get_source_inst_name() {
            signal.push_str(&format!("{}.", source_inst_name));
        }

        // Add stream name
        signal.push_str(self.get_source_stream_name());

        // Connector
        signal.push_str(" -- ");

        // If the signal is to an instance, add the instance name
        if let Some(dest_inst_name) = self.get_dest_inst_name() {
            signal.push_str(&format!("{}.", dest_inst_name));
        }

        // Add stream name
        signal.push_str(self.get_dest_stream_name());

        write!(f, "{};", signal)
    }
}

impl TilSignal {
    pub fn get_source_inst_name(&self) -> Option<&String> {
        match self {
            TilSignal::Input{..} => None,
            TilSignal::Intermediate{source_inst_name, ..} => Some(source_inst_name),
            TilSignal::Output{source_inst_name, ..} => Some(source_inst_name),
        }
    }

    pub fn get_source_stream_name(&self) -> &String {
        match self {
            TilSignal::Input{source_stream_name, ..} => source_stream_name,
            TilSignal::Intermediate{source_stream_name, ..} => source_stream_name,
            TilSignal::Output{source_stream_name, ..} => source_stream_name,
        }
    }

    pub fn get_dest_inst_name(&self) -> Option<&String> {
        match self {
            TilSignal::Input{dest_inst_name, ..} => Some(dest_inst_name),
            TilSignal::Intermediate{dest_inst_name, ..} => Some(dest_inst_name),
            TilSignal::Output{..} => None,
        }
    }

    pub fn get_dest_stream_name(&self) -> &String {
        match self {
            TilSignal::Input{dest_stream_name, ..} => dest_stream_name,
            TilSignal::Intermediate{dest_stream_name, ..} => dest_stream_name,
            TilSignal::Output{dest_stream_name, ..} => dest_stream_name,
        }
    }

    pub fn td(&self) -> String {
        let mut signal = String::new();

        // If the signal is from an instance, add the instance name
        if let Some(source_inst_name) = self.get_source_inst_name() {
            signal.push_str(&format!("    {}.", source_inst_name));
        } else {
            signal.push_str(&"self.".to_string());
        }

        // Add stream name
        signal.push_str(self.get_source_stream_name());

        // Connector
        signal.push_str(" => ");

        // If the signal is to an instance, add the instance name
        if let Some(dest_inst_name) = self.get_dest_inst_name() {
            signal.push_str(&format!("{}.", dest_inst_name));
        } else {
            signal.push_str(&"self.".to_string());
        }

        // Add stream name
        signal.push_str(self.get_dest_stream_name());

        format!("{};", signal)
    }
}