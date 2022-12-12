use json::JsonValue;

use super::{JsonComponent, JsonType};

// Analyze a record of the JSON object
pub fn analyze_record(key: &str, element: &JsonValue, outer_nesting: u16, inner_nesting: u16) -> (Option<JsonComponent>, u16) {
    let (child, new_inner_nesting) = analyze_element(element, outer_nesting, inner_nesting);

    match child {
        Some(child) => 
            (
                Some(
                    JsonComponent::Key {
                        name: key.to_string(),
                        outer_nested: outer_nesting + 1,
                        value: Some(Box::new(child)),
                    }
            ), new_inner_nesting),
        None => (None, new_inner_nesting)
    }
}

// Analyze the element and recursively call itself if it is an object or array to find nested elements
pub fn analyze_element(element: &JsonValue, outer_nesting: u16, inner_nesting: u16) -> (Option<JsonComponent>, u16) {
    match element {
        // Element has string type
        JsonValue::Short(_) | JsonValue::String(_) => 
            (
                Some(
                    JsonComponent::Value {
                        data_type: JsonType::String,
                        outer_nested: outer_nesting, // Strings don't increase the nesting level since the input is a string
                    }
                ), 
                // Types don't increase the nesting level
                inner_nesting
            ),
        // Element has integer type
        JsonValue::Number(_) => 
            (
                Some(
                    JsonComponent::Value {
                        data_type: JsonType::Integer,
                        outer_nested: outer_nesting + 1,
                    }
                ), 
                // Types don't increase the nesting level
                inner_nesting
            ),
        // Element has boolean type
        JsonValue::Boolean(_) => 
            (
                Some(
                    JsonComponent::Value {
                        data_type: JsonType::Boolean,
                        outer_nested: outer_nesting + 1,
                    }
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
            let (child, new_inner_nesting) = analyze_element(child_element, outer_nesting + 1, inner_nesting);

            // Return the array with the child element
            (
                Some(
                    JsonComponent::Array {
                        outer_nested: outer_nesting + 1,
                        inner_nested: new_inner_nesting,
                        value: match child {
                            Some(component) => Some(Box::new(component)),
                            None => None,
                        }
                    }
                ),
                // An array increases the inner nesting by 1
                new_inner_nesting + 1
            )
        },
        // Element is an object
        JsonValue::Object(_) => {
            let mut children: Vec<JsonComponent> = Vec::new();
            let mut new_inner_nesting = Vec::new();

            // Analyze all the records of the object
            for key in element.entries() {
                // Analyze the record
                let (child, ret_inner_nesting) = analyze_record(key.0, key.1, outer_nesting + 1, inner_nesting);
                
                // Check if the record is not empty
                match child {
                    Some(component) => {
                        children.push(component)
                    },
                    _ => (),
                } 

                // Save the inner nesting level of the record
                new_inner_nesting.push(ret_inner_nesting);
            }

            // Take the maximum inner nesting of the object's records
            let max_inner_nesting = *(new_inner_nesting.iter().max().unwrap());

            // Return the object with the children
            (
                Some(
                    JsonComponent::Object {
                        outer_nested: outer_nesting + 1,
                        inner_nested: max_inner_nesting,
                        records: children,
                    }
                ),
                // An object increases the inner nesting by 1
                max_inner_nesting + 1
            )
        },
        JsonValue::Null => (None, inner_nesting),
    }
}