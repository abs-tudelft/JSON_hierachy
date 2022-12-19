use super::TypeReg;

impl TypeReg {
    pub fn new() -> TypeReg {
        TypeReg {
            type_list: Vec::new(),
        }
    }

    /// Register a new data type
    /// 
    /// Returns true if the data type was already registered
    pub fn register(&mut self, data_type: &str) -> bool {
        let exists = self.type_list.contains(&String::from(data_type));

        if !exists {
            self.type_list.push(String::from(data_type));
        }

        exists
    }
}