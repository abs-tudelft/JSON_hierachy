use json::JsonValue;

use crate::analysis::components::{JsonComponent, JsonType, Record, Key, Value, Array, Object, Matcher};

/**********************************************************************************
 * Set of functions to analyze the parsed JSON object into a component structure  *
 * which can be used to generate HDL code.                                        *
 **********************************************************************************/

pub fn analyze(root: &JsonValue) -> Option<JsonComponent> {
    let (root_component, _) = analyze_element(root, 0, 0);
    root_component
}

// Analyze a record of the JSON object
fn analyze_record(key: &str, element: &JsonValue, outer_nesting: usize, inner_nesting: usize) -> (Option<Record>, usize) {
    let (child, new_inner_nesting) = analyze_element(element, outer_nesting + 1, inner_nesting);

    (
        Some(
            Record::new( 
                outer_nesting + 1,
                new_inner_nesting,
                Key::new(
                        Matcher::new(key.to_string(), outer_nesting + 2),
                        outer_nesting + 2,
                        child.map(Box::new)
                    )
            )
        ), 
    new_inner_nesting + 1)    
}

// Analyze the element and recursively call itself if it is an object or array to find nested elements
fn analyze_element(element: &JsonValue, outer_nesting: usize, inner_nesting: usize) -> (Option<JsonComponent>, usize) {
    match element {
        // Element has string type
        JsonValue::Short(_) | JsonValue::String(_) => 
            (
                Some(
                    JsonComponent::Value(
                        Value::new(
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
            let (child, new_inner_nesting) = analyze_element(child_element, outer_nesting + 1, inner_nesting);

            // Return the array with the child element
            (
                Some(
                    JsonComponent::Array(
                        Array::new(
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
            let mut children: Vec<Record> = Vec::new();
            let mut new_inner_nesting = Vec::new();

            // Analyze all the records of the object
            for key in element.entries() {
                // Analyze the record
                let (child, ret_inner_nesting) = analyze_record(key.0, key.1, outer_nesting, inner_nesting);
                
                // Push record if it is not None
                if let Some(component) = child {
                    children.push(component);
                }

                // Save the inner nesting level of the record
                new_inner_nesting.push(ret_inner_nesting);
            }

            // Take the maximum inner nesting of the object's records
            let max_inner_nesting = *(new_inner_nesting.iter().max().unwrap());

            // Return the object with the children
            (
                Some(
                    JsonComponent::Object(
                        Object::new(children)
                    )
                ),
                // An object increases the inner nesting by 1
                max_inner_nesting + 1
            )
        },
        JsonValue::Null => (None, inner_nesting),
    }
}
