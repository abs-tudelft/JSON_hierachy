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

    let root = analyze_element(&p, 0);

    println!("{:?}", root);
}

fn analyze_record(key: &str, element: &json::JsonValue, outer_nesting: u16) -> Option<JsonComponent> {
    let child = analyze_element(element, outer_nesting);

    match child {
        Some(child) => 
            Some(JsonComponent::Record(
                Record {
                    name: key.to_string(),
                    outer_nested: outer_nesting,
                    inner_nested: 0,
                    child: Some(Box::new(child)),
                }
            )
        ),
        None => None,
    }
}

fn analyze_element(element: &json::JsonValue, outer_nesting: u16) -> Option<JsonComponent> {
    match element {
        json::JsonValue::Short(_) | json::JsonValue::String(_) => 
            Some(
                JsonComponent::Value(
                    Value {
                        data_type: JsonType::String,
                        outer_nested: outer_nesting + 1,
                    }
                )
            ),
        json::JsonValue::Number(_) => 
            Some(
                JsonComponent::Value(
                    Value {
                        data_type: JsonType::Integer,
                        outer_nested: outer_nesting + 1,
                    }
                )
            ),
        json::JsonValue::Boolean(_) => 
            Some(
                JsonComponent::Value(
                    Value {
                        data_type: JsonType::Boolean,
                        outer_nested: outer_nesting + 1,
                    }
                )
            ),
        json::JsonValue::Array(arr) => {
            let child_element = &arr[0];
            let child = analyze_element(child_element, outer_nesting + 1);

            Some(
                JsonComponent::Array(
                    Array {
                        outer_nested: outer_nesting + 1,
                        inner_nested: 0,
                        child: match child {
                            Some(component) => Some(Box::new(component)),
                            None => None,
                        }
                    }
                )
            )
        },
        json::JsonValue::Object(_) => {
            let mut children: Vec<Record> = Vec::new();
            for key in element.entries() {
                match analyze_record(key.0, key.1, outer_nesting + 1) {
                    Some(JsonComponent::Record(record)) => children.push(record),
                    _ => (),
                } 
            }

            Some(
                JsonComponent::Object(
                    Object {
                        outer_nested: outer_nesting + 1,
                        inner_nested: 0,
                        children: children,
                    }
                )
            )
        },
        json::JsonValue::Null => None,
    }
}