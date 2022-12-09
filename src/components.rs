use core::fmt;

#[derive(Debug)]
pub enum JsonType {
    String,
    Integer,
    Boolean,
}

pub enum JsonComponent {
    Value {
        data_type: JsonType, 
        outer_nested: u16
    },
    Array{
        outer_nested: u16,
        inner_nested: u16,
        value: Option<Box<JsonComponent>>
    },
    Object {
        outer_nested: u16,
        inner_nested: u16,
        records: Vec<JsonComponent>
    },
    Key {
        name: String,
        outer_nested: u16,
        value: Option<Box<JsonComponent>>
    },
}

impl JsonComponent {
    pub fn to_vhdl(&self) -> String {
        match self {
            JsonComponent::Value { data_type, outer_nested } => {
                let mut vhdl = String::new();
                vhdl.push_str(&format!("{}: {:?}", outer_nested, data_type));
                vhdl
            },
            JsonComponent::Array { outer_nested, inner_nested, value: _ } => {
                let mut vhdl = String::new();
                vhdl.push_str(&format!("{}: Array({}) of ", outer_nested, inner_nested));
                vhdl
            },
            JsonComponent::Key { name, outer_nested: _, value: _ } => {
                let mut vhdl = String::new();
                vhdl.push_str(&format!("Key({})", name));
                vhdl
            },
            _ => "".to_string(),
        }
    }
}

// Implementation of how to render the component hierachy on the console
impl fmt::Display for JsonComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = String::new();

        match self {
            JsonComponent::Object { outer_nested: _, inner_nested:_, records } => {
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
        }
    }
}