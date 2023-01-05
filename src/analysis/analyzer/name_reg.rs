use std::collections::HashMap;

pub struct NameReg {
    name_map: HashMap<String, usize>,
}

impl NameReg {
    pub fn new() -> NameReg {
        NameReg {
            name_map: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, nesting_level: usize) -> String {
        let mut pref_name = String::from(name);

        // Add nesting level to prefered name
        pref_name.push_str(&format!("_L{}_", nesting_level)); 


        let mut registered_name = pref_name.clone();

        // Check if prefered name is already in the map
        if self.name_map.contains_key(&pref_name) {
            // Get current count for name
            let count = self.name_map.get_mut(&pref_name).unwrap();
            registered_name.push_str(&format!("{:#02}", count));

            // Increment count
            *count += 1;
        } else {
            // If not, insert it with a value of 1
            registered_name.push_str(&String::from("00"));
            self.name_map.insert(String::from(&pref_name), 1);
        }

        registered_name
    }
}