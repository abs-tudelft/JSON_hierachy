use core::fmt;

pub enum JsonType {
    String,
    Integer,
    Boolean,
}


pub enum JsonComponent {
    Value {
        dataType: JsonType, 
        outer_nested: u16
    },
    Array{
        outer_nested: u16,
        inner_nested: u16,
        child: Option<Box<JsonComponent>>
    },
    Object {
        outer_nested: u16,
        inner_nested: u16,
        children: Vec<JsonComponent>
    },
    Record {
        name: String,
        outer_nested: u16,
        inner_nested: u16,
        child: Option<Box<JsonComponent>>
    },
}

impl fmt::Display for JsonComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = String::new();

        match self {
            JsonComponent::Object { outer_nested: _, inner_nested:_, children } => {
                for child in children {
                    output.push_str(&format!("{}", child));
                }

                write!(f, "{}", output)
            },
            JsonComponent::Array { outer_nested, inner_nested: _, child } => {
                for _ in 0..outer_nested-1 {
                    output.push_str("\t");
                }

                output.push_str("Array\n");

                match child {
                    Some(ref ref_child) => output.push_str(&format!("{}", ref_child)),
                    None => output.push_str("Empty"),
                };

                write!(f, "{}", output)
            },
            JsonComponent::Value { dataType, outer_nested } => {
                for _ in 0..outer_nested-1 {
                    output.push_str("\t");
                }

                match dataType {
                    JsonType::String => output.push_str("String"),
                    JsonType::Integer => output.push_str("Integer"),
                    JsonType::Boolean => output.push_str("Boolean"),
                };

                write!(f, "{}", output)
            }
            JsonComponent::Record { name, outer_nested, inner_nested: _, child } => {
                let mut output: String = String::new();

                for _ in 0..outer_nested-1 {
                    output.push_str("\t");
                }

                output.push_str(&format!("Record: {}\n", name));

                match child {
                    Some(ref ref_child) => {
                        output.push_str(&format!("{}", ref_child));
                    },
                    None => {
                        output.push_str(&format!("Empty"));
                    }
                }
                    
                write!(f, "{}\n", output)
            },
        }
    }
}