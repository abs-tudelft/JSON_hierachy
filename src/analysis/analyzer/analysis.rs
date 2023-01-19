use json::JsonValue;

use crate::analysis::components::{JsonComponent, JsonType, Record, Key, Value, Array, Matcher, Generatable};

use super::Analyzer;

impl Analyzer {
    // Analyze a record of the JSON object
    // Which results in the creation of 3 components:
    // A matcher, a key and a record
    pub fn analyze_record(&mut self, key: &str, element: &JsonValue, outer_nesting: usize, inner_nesting: usize) -> (Option<Key>, usize) {
        let (child, new_inner_nesting) = self.analyze_element(element, outer_nesting + 1, inner_nesting);

        let key_name = self.name_reg.register("key_parser", outer_nesting + 2);
        let matcher_name = self.name_reg.register(&format!("{}_matcher", key), outer_nesting + 2);

        // Create a components
        let matcher = Matcher::new(&matcher_name, &key_name, key.to_string(), outer_nesting + 2);
        let key = Key::new(&key_name, matcher.clone(), outer_nesting + 2, child.map(Box::new));
        
        // Convert to TilComponent
        let matcher_component = matcher.to_til_streamlet(&self.gen_params);
        let key_component = key.to_til_streamlet(&self.gen_params);

        // Add components to entity list
        self.entity_list.push(matcher_component);
        self.entity_list.push(key_component);

        // Register types
        self.type_manager.register_from_component(&matcher);
        self.type_manager.register_from_component(&key);

        // Add signals to signal list
        self.signal_list.append(&mut matcher.get_outgoing_signals());
        self.signal_list.append(&mut key.get_outgoing_signals());

        // Add entity to file manager
        self.file_manager.add_entity(matcher.get_file_type(), matcher.get_name());
        self.file_manager.add_entity(key.get_file_type(), key.get_name());


        // Return the key and keep the same inner nesting level
        (Some(key), new_inner_nesting)  
    }

    // Analyze the element and recursively call itself if it is an object or array to find nested elements
    pub fn analyze_element(&mut self, element: &JsonValue, outer_nesting: usize, inner_nesting: usize) -> (Option<JsonComponent>, usize) {
        let (component, new_inner_nesting) = match element {
            // Element has string type
            JsonValue::Short(_) | JsonValue::String(_) => 
                (
                    Some(
                        JsonComponent::Value(
                            Value::new(
                                &self.name_reg.register("string_parser", outer_nesting),
                                JsonType::String,
                                outer_nesting, // Strings don't increase the nesting level since the input is a string
                            )
                        )
                    ), 
                    // Types don't increase the nesting level
                    inner_nesting
                ),
            // Element has integer type
            JsonValue::Number(_) => 
                (
                    Some(
                        JsonComponent::Value(
                            Value::new(
                                &self.name_reg.register("int_parser", outer_nesting + 1),
                                JsonType::Integer,
                                outer_nesting + 1,
                            )
                        )
                    ), 
                    // Types don't increase the nesting level
                    inner_nesting
                ),
            // Element has boolean type
            JsonValue::Boolean(_) => 
                (
                    Some(
                        JsonComponent::Value(
                            Value::new(
                                &self.name_reg.register("bool_parser", outer_nesting + 1),
                                JsonType::Boolean,
                                outer_nesting + 1,
                            )
                        )                               
                    ), 
                    // Types don't increase the nesting level
                    inner_nesting
                ),
            // Element is an array
            JsonValue::Array(arr) => {
                // If the array is empty, return None
                if arr.is_empty() {
                    return (None, inner_nesting + 1);
                }

                // Get the first element of the array to determine the type of the array
                let child_element = &arr[0];
                let (child, new_inner_nesting) = self.analyze_element(child_element, outer_nesting + 1, inner_nesting);

                // Return the array with the child element
                (
                    Some(
                        JsonComponent::Array(
                            Array::new(
                                &self.name_reg.register("array_parser", outer_nesting + 1),
                                outer_nesting + 1,
                                new_inner_nesting,
                                child.map(Box::new)
                            )
                        )
                    ),
                    // An array increases the inner nesting by 1
                    new_inner_nesting + 1
                )
            },
            // Element is an object
            JsonValue::Object(_) => {
                let mut children: Vec<Key> = Vec::new();
                let mut new_inner_nesting = Vec::new();

                // Analyze all the records of the object
                for key in element.entries() {
                    // Analyze the record
                    let (child, ret_inner_nesting) = self.analyze_record(key.0, key.1, outer_nesting, inner_nesting);
                    
                    // Push record if it is not None
                    if let Some(key) = child {
                        children.push(key);
                    }

                    // Save the inner nesting level of the record
                    new_inner_nesting.push(ret_inner_nesting);
                }

                // Take the maximum inner nesting of the object's records
                let max_inner_nesting = *(new_inner_nesting.iter().max().unwrap());

                // Return the object with the children
                (
                    Some(
                        JsonComponent::Record(
                            Record::new(
                                &self.name_reg.register("record_parser", outer_nesting + 1), 
                                outer_nesting + 1, 
                                max_inner_nesting, 
                                children
                            )
                        )
                    ),
                    // An object increases the inner nesting by 1
                    max_inner_nesting + 1
                )
            },
            JsonValue::Null => (None, inner_nesting),
        };

        // Check if there is a component
        if let Some(component) = &component {
            // Check if the component is generatable
            let gen_component = component.get_generatable();

            // Convert to TilStreamlet
            let til_streamlet = gen_component.to_til_streamlet(&self.gen_params);

            // Add components to entity list
            self.entity_list.push(til_streamlet);

            // Register types
            self.type_manager.register_from_component(gen_component);

            // Add signals to signal list
            self.signal_list.append(&mut gen_component.get_outgoing_signals());

            // Add entity to file manager
            self.file_manager.add_entity(gen_component.get_file_type(), gen_component.get_name());
        }

        // Return the component and the new inner nesting level
        (component, new_inner_nesting)
    }
}