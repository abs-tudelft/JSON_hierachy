use std::collections::HashMap;

use super::NameReg;

impl NameReg {
    pub fn new() -> NameReg {
        NameReg {
            name_map: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, nesting_level: u16) -> String {
        let mut registered_name = String::from(name);

        // Add nesting level to name
        registered_name.push_str(format!("_L{}_", nesting_level).as_str());

        // Check if name is already in the map
        if self.name_map.contains_key(name) {
            // Get current count for name
            let count = self.name_map.get_mut(name).unwrap();
            registered_name.push_str(count.to_string().as_str());

            // Increment count
            *count += 1;
        } else {
            // If not, insert it with a value of 1
            registered_name.push_str(&String::from("0"));
            self.name_map.insert(String::from(name), 1);
        }

        registered_name
    }
}