#[derive(Debug)]
enum JsonType {
    String,
    Integer,
    Boolean,
}

#[derive(Debug)]
enum JsonComponent {
    Value(Value),
    Array(Array),
    Object(Object),
    Record(Record),
}

#[derive(Debug)]
struct Value {
    data_type: JsonType,
    outer_nested: u16,
}

#[derive(Debug)]
struct Array {
    outer_nested: u16,
    inner_nested: u16,
    child: Option<Box<JsonComponent>>
}

#[derive(Debug)]
struct Record {
    name: String,
    outer_nested: u16,
    inner_nested: u16,
    child: Option<Box<JsonComponent>>
}

#[derive(Debug)]
struct Object {
    outer_nested: u16,
    inner_nested: u16,
    children: Vec<Record>
}

fn main() {
    let data = r#"
    {
        "voltage":
            [{"voltage":1128},{"voltage":1213},{"voltage":1850}]
     }
     "#;

    //  let data = r#"
    //  {
    //     "voltage":
    //         [1128,1213,1850,429]
    //  }
    //  "#;

    // Deserialize the JSON string
    let p = json::parse(data).unwrap();

    let (root, _) = analyze_element(&p, 0, 0);

    println!("{:?}", root);
}

// Analyze a record of the JSON object
fn analyze_record(key: &str, element: &json::JsonValue, outer_nesting: u16, inner_nesting: u16) -> (Option<JsonComponent>, u16) {
    let (child, new_inner_nesting) = analyze_element(element, outer_nesting, inner_nesting);

    match child {
        Some(child) => 
            (
                Some(JsonComponent::Record(
                    Record {
                        name: key.to_string(),
                        outer_nested: outer_nesting,
                        inner_nested: new_inner_nesting,
                        child: Some(Box::new(child)),
                    }
                )
            ), new_inner_nesting),
        None => (None, new_inner_nesting)
    }
}

// Analyze the element and recursively call itself if it is an object or array to find nested elements
fn analyze_element(element: &json::JsonValue, outer_nesting: u16, inner_nesting: u16) -> (Option<JsonComponent>, u16) {
    match element {
        // Element has string type
        json::JsonValue::Short(_) | json::JsonValue::String(_) => 
            (
                Some(
                    JsonComponent::Value(
                        Value {
                            data_type: JsonType::String,
                            outer_nested: outer_nesting + 1,
                        }
                    )
                ), 
                inner_nesting
            ),
        // Element has integer type
        json::JsonValue::Number(_) => 
            (
                Some(
                    JsonComponent::Value(
                        Value {
                            data_type: JsonType::Integer,
                            outer_nested: outer_nesting + 1,
                        }
                    )
                ), 
                inner_nesting
            ),
        // Element has boolean type
        json::JsonValue::Boolean(_) => 
            (
                Some(
                    JsonComponent::Value(
                        Value {
                            data_type: JsonType::Boolean,
                            outer_nested: outer_nesting + 1,
                        }
                    )
                ), 
                inner_nesting
            ),
        // Element is an array
        json::JsonValue::Array(arr) => {
            // If the array is empty, return None
            if arr.is_empty() {
                return (None, inner_nesting);
            }

            // Get the first element of the array to determine the type of the array
            let child_element = &arr[0];
            let (child, new_inner_nesting) = analyze_element(child_element, outer_nesting + 1, inner_nesting);

            // Return the array with the child element
            (
                Some(
                    JsonComponent::Array(
                        Array {
                            outer_nested: outer_nesting + 1,
                            inner_nested: new_inner_nesting,
                            child: match child {
                                Some(component) => Some(Box::new(component)),
                                None => None,
                            }
                        }
                    )
                ),
                new_inner_nesting + 1
            )
        },
        // Element is an object
        json::JsonValue::Object(_) => {
            // Analyze all the records of the object
            let mut children: Vec<Record> = Vec::new();
            let mut new_inner_nesting = Vec::new();
            for key in element.entries() {
                let (child, ret_inner_nesting) = analyze_record(key.0, key.1, outer_nesting + 1, inner_nesting);
                match child {
                    Some(JsonComponent::Record(record)) => children.push(record),
                    _ => (),
                } 
                new_inner_nesting.push(ret_inner_nesting);
            }

            let max_inner_nesting = *(new_inner_nesting.iter().max().unwrap());

            // Return the object with the children
            (
                Some(
                    JsonComponent::Object(
                        Object {
                            outer_nested: outer_nesting + 1,
                            inner_nested: max_inner_nesting,
                            children: children,
                        }
                    )
                ),
                max_inner_nesting + 1
            )
        },
        json::JsonValue::Null => (None, inner_nesting),
    }
}