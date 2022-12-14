use crate::analysis::{GeneratorParams, gen_tools::GenTools};

use super::{Object, JsonComponent, Generatable, Record};

impl Object {
    pub fn new(records: Vec<Record>) -> Object {
        Object {
            records
        }
    }
}

impl Generatable for Object {
    fn to_til(&self, _name_reg: &mut GenTools, _gen_params: &GeneratorParams) -> String {
        String::from("")
    }

    fn to_graph_node(&self) -> Option<String> {
        None
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        let mut children: Vec<JsonComponent> = Vec::new();

        for record in &self.records {
            children.push(JsonComponent::Record(record.clone()));
        }

        return children;
    }
}