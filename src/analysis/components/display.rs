use core::fmt;

use super::{JsonComponent, JsonType};

/**********************************************************************************
 * Implementation of how to render the component hierachy on the console          *
 * (The dot visualization in the generator is more clear)                         *
 **********************************************************************************/


impl fmt::Display for JsonComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = String::new();

        match self {
            JsonComponent::Object { records } => {
                for child in records {
                    output.push_str(&format!("{}", child));
                }

                write!(f, "{}", output)
            },
            JsonComponent::Array { outer_nested, inner_nested: _, value } => {
                for _ in 0..outer_nested-1 {
                    output.push_str("\t");
                }

                output.push_str("Array\n");

                match value {
                    Some(ref ref_child) => output.push_str(&format!("{}", ref_child)),
                    None => output.push_str("Empty"),
                };

                write!(f, "{}", output)
            },
            JsonComponent::Value { data_type, outer_nested } => {
                for _ in 0..outer_nested-1 {
                    output.push_str("\t");
                }

                match data_type {
                    JsonType::String => output.push_str("String"),
                    JsonType::Integer => output.push_str("Integer"),
                    JsonType::Boolean => output.push_str("Boolean"),
                };

                write!(f, "{}", output)
            }
            JsonComponent::Key { name, outer_nested, value } => {
                let mut output: String = String::new();

                for _ in 0..outer_nested-1 {
                    output.push_str("\t");
                }

                output.push_str(&format!("Key: {}\n", name));

                match value {
                    Some(ref ref_child) => {
                        output.push_str(&format!("{}", ref_child));
                    },
                    None => {
                        output.push_str(&format!("Empty"));
                    }
                }
                    
                write!(f, "{}\n", output)
            },
            JsonComponent::Record { outer_nested, inner_nested: _, key } => {
                let mut output: String = String::new();

                for _ in 0..outer_nested-1 {
                    output.push_str("\t");
                }

                output.push_str("Record\n");

                output.push_str(&format!("{}", key));

                write!(f, "{}", output)
            }
        }
    }
}