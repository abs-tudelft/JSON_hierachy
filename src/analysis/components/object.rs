use super::{Object, JsonComponent, Record, JsonComponentValue};

impl Object {
    pub fn new(records: Vec<Record>) -> Object {
        Object {
            records
        }
    }
}

impl JsonComponentValue for Object {
    fn to_graph_node(&self) -> Option<String> {
        None
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        let mut children: Vec<JsonComponent> = Vec::new();

        for record in &self.records {
            children.push(JsonComponent::Record(record.clone()));
        }

        children
    }

    fn num_children(&self) -> usize {
        self.records.len()
    }
}