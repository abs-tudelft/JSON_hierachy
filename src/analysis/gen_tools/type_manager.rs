use std::collections::HashMap;

use indoc::formatdoc;

use crate::analysis::types::TilStreamType;

use super::TypeManager;

impl TypeManager {
    pub fn new() -> TypeManager {
        TypeManager {
            type_list: HashMap::new(),
        }
    }

    /// Register a new data type
    /// 
    /// Returns true if the data type was already registered
    pub fn register(&mut self, stream_type: TilStreamType) {
        let type_name: &str = stream_type.get_name();

        if !self.does_type_exist(type_name) {
            self.type_list.insert(type_name.to_owned(), stream_type);
        }
    }

    pub fn get_type(&self, data_type: &str) -> Option<&TilStreamType> {
        self.type_list.get(&String::from(data_type))
    }

    pub fn does_type_exist(&self, data_type: &str) -> bool {
        self.type_list.contains_key(&String::from(data_type))
    }

    pub fn generate_type_defs(&self) -> String {
        let mut type_defs = String::new();

        for (type_name, stream_type) in &self.type_list {
            type_defs.push_str(
                &formatdoc!(
                    "
                    type {} = Stream (
                        data: Bits({}),
                        throughput: {},
                        dimensionality: {},
                        synchronicity: {:?},
                        complexity: {},
                    );
                    ",
                    type_name,
                    stream_type.data_bits,
                    stream_type.throughput,
                    stream_type.dimensionality,
                    stream_type.synchronicity,
                    stream_type.complexity,
                )
            );

            type_defs.push('\n');
        }

        type_defs
    }
}