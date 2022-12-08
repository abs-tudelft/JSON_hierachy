use core::fmt;
use std::io::Write;

#[derive(Debug)]
pub enum JsonType {
    String,
    Integer,
    Boolean,
}


pub enum JsonComponent {
    Value {
        data_type: JsonType, 
        outer_nested: u16
    },
    Array{
        outer_nested: u16,
        inner_nested: u16,
        value: Option<Box<JsonComponent>>
    },
    Object {
        outer_nested: u16,
        inner_nested: u16,
        records: Vec<JsonComponent>
    },
    Key {
        name: String,
        outer_nested: u16,
        value: Option<Box<JsonComponent>>
    },
}

type Nd<'a> = (usize, &'a str);
type Ed<'a> = (Nd<'a>, Nd<'a>);
struct Graph { nodes: Vec<String>, edges: Vec<(usize,usize)> }

impl JsonComponent {
    pub fn to_vhdl(&self) -> String {
        match self {
            JsonComponent::Value { data_type, outer_nested } => {
                let mut vhdl = String::new();
                vhdl.push_str(&format!("{}: {:?}", outer_nested, data_type));
                vhdl
            },
            JsonComponent::Array { outer_nested, inner_nested, value: _ } => {
                let mut vhdl = String::new();
                vhdl.push_str(&format!("{}: Array({}) of ", outer_nested, inner_nested));
                vhdl
            },
            JsonComponent::Key { name, outer_nested: _, value: _ } => {
                let mut vhdl = String::new();
                vhdl.push_str(&format!("Key({})", name));
                vhdl
            },
            _ => "".to_string(),
        }
    }

    fn update_graph(&self, parent_id: Option<usize>, graph: &mut Graph) {
        match self {
            JsonComponent::Value { data_type, outer_nested } => {
                graph.nodes.push(format!("{:?} parser\nO: {}", data_type, outer_nested));
                let id = graph.nodes.len() - 1;

                if let Some(parent_id) = parent_id {
                    graph.edges.push((parent_id, id));
                }
            },
            JsonComponent::Array { outer_nested, inner_nested, value } => {
                graph.nodes.push(format!("Array parser\nO: {}, I: {}", outer_nested, inner_nested));
                let id = graph.nodes.len() - 1;

                if let Some(parent_id) = parent_id {
                    graph.edges.push((parent_id, id));
                }

                if let Some(value) = value {
                    value.update_graph(Some(id), graph);
                }
            },
            JsonComponent::Object { outer_nested, inner_nested, records } => {
                graph.nodes.push(format!("Object parser\nO: {}, I: {}", outer_nested, inner_nested));
                let id = graph.nodes.len() - 1;

                if let Some(parent_id) = parent_id {
                    graph.edges.push((parent_id, id));
                }

                for record in records {
                    record.update_graph(Some(id), graph);
                }
            },
            JsonComponent::Key { name, outer_nested, value } => {
                graph.nodes.push(format!("Key filter\nMatch: \"{}\"\nO: {}", name, outer_nested));
                let id = graph.nodes.len() - 1;

                if let Some(parent_id) = parent_id {
                    graph.edges.push((parent_id, id));
                }

                if let Some(value) = value {
                    value.update_graph(Some(id), graph);
                }
            },
        }
    }

    pub fn to_dot<W: Write>(&self, output_file: &mut W) {
        let mut graph: Graph = Graph{nodes: Vec::new(), edges: Vec::new()};
        self.update_graph(None, &mut graph);
        dot::render(&graph, output_file).unwrap()
    }
}

impl<'a> dot::Labeller<'a, Nd<'a>, Ed<'a>> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example3").unwrap() }
    fn node_id(&'a self, n: &Nd<'a>) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n.0)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &Nd<'b>) -> dot::LabelText<'b> {
        let &(i, _) = n;
        dot::LabelText::LabelStr(std::borrow::Cow::Borrowed(self.nodes[i].as_str()))
    }
}

impl<'a> dot::GraphWalk<'a, Nd<'a>, Ed<'a>> for Graph {
    fn nodes(&'a self) -> dot::Nodes<'a,Nd<'a>> {
        self.nodes.iter().map(|s| &s[..]).enumerate().collect()
    }
    fn edges(&'a self) -> dot::Edges<'a,Ed<'a>> {
        self.edges.iter()
            .map(|&(i,j)|((i, &self.nodes[i][..]),
                          (j, &self.nodes[j][..])))
            .collect()
    }
    fn source(&self, e: &Ed<'a>) -> Nd<'a> { e.0 }
    fn target(&self, e: &Ed<'a>) -> Nd<'a> { e.1 }
}

impl fmt::Display for JsonComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = String::new();

        match self {
            JsonComponent::Object { outer_nested: _, inner_nested:_, records } => {
                for child in records {
                    output.push_str(&format!("{}", child));
                }

                write!(f, "{}", output)
            },
            JsonComponent::Array { outer_nested, inner_nested: _, value } => {
                for _ in 0..outer_nested-1 {
                    output.push_str("\t");
                }

                output.push_str("Array\n");

                match value {
                    Some(ref ref_child) => output.push_str(&format!("{}", ref_child)),
                    None => output.push_str("Empty"),
                };

                write!(f, "{}", output)
            },
            JsonComponent::Value { data_type, outer_nested } => {
                for _ in 0..outer_nested-1 {
                    output.push_str("\t");
                }

                match data_type {
                    JsonType::String => output.push_str("String"),
                    JsonType::Integer => output.push_str("Integer"),
                    JsonType::Boolean => output.push_str("Boolean"),
                };

                write!(f, "{}", output)
            }
            JsonComponent::Key { name, outer_nested, value } => {
                let mut output: String = String::new();

                for _ in 0..outer_nested-1 {
                    output.push_str("\t");
                }

                output.push_str(&format!("Key: {}\n", name));

                match value {
                    Some(ref ref_child) => {
                        output.push_str(&format!("{}", ref_child));
                    },
                    None => {
                        output.push_str(&format!("Empty"));
                    }
                }
                    
                write!(f, "{}\n", output)
            },
        }
    }
}