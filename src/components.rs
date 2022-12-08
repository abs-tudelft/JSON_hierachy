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
        dataType: JsonType, 
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
        value: Option<Box<JsonComponent>>
    },
}

type Nd<'a> = (usize, &'a str);
type Ed<'a> = (Nd<'a>, Nd<'a>);
struct Graph { nodes: Vec<String>, edges: Vec<(usize,usize)> }

impl JsonComponent {
    pub fn to_vhdl(&self) -> String {
        match self {
            JsonComponent::Value { dataType, outer_nested } => {
                let mut vhdl = String::new();
                vhdl.push_str(&format!("{}: {:?}", outer_nested, dataType));
                vhdl
            },
            JsonComponent::Array { outer_nested, inner_nested, value } => {
                let mut vhdl = String::new();
                vhdl.push_str(&format!("{}: Array({}) of ", outer_nested, inner_nested));
                vhdl
            },
            JsonComponent::Key { name, value } => {
                let mut vhdl = String::new();
                vhdl.push_str(&format!("Key({})", name));
                vhdl
            },
            _ => "".to_string(),
        }
    }

    fn update_graph(&self, parentID: Option<usize>, graph: &mut Graph) {
        match self {
            JsonComponent::Value { dataType, outer_nested } => {
                graph.nodes.push(format!("{:?} parser", dataType));
                let id = graph.nodes.len() - 1;

                if let Some(parentID) = parentID {
                    graph.edges.push((parentID, id));
                }
            },
            JsonComponent::Array { outer_nested, inner_nested, value } => {
                graph.nodes.push(format!("Array parser"));
                let id = graph.nodes.len() - 1;

                if let Some(parentID) = parentID {
                    graph.edges.push((parentID, id));
                }

                if let Some(value) = value {
                    value.update_graph(Some(id), graph);
                }
            },
            JsonComponent::Object { outer_nested, inner_nested, records } => {
                graph.nodes.push(format!("Record parser"));
                let id = graph.nodes.len() - 1;

                if let Some(parentID) = parentID {
                    graph.edges.push((parentID, id));
                }

                for record in records {
                    record.update_graph(Some(id), graph);
                }
            },
            JsonComponent::Key { name, value } => {
                graph.nodes.push(format!("Key filter"));
                let id = graph.nodes.len() - 1;

                if let Some(parentID) = parentID {
                    graph.edges.push((parentID, id));
                }

                if let Some(value) = value {
                    value.update_graph(Some(id), graph);
                }
            },
        }
    }

    pub fn to_dot<W: Write>(&self, outputFile: &mut W) {
        let mut graph: Graph = Graph{nodes: Vec::new(), edges: Vec::new()};
        self.update_graph(None, &mut graph);
        dot::render(&graph, outputFile).unwrap()
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
            JsonComponent::Value { dataType, outer_nested } => {
                for _ in 0..outer_nested-1 {
                    output.push_str("\t");
                }

                match dataType {
                    JsonType::String => output.push_str("String"),
                    JsonType::Integer => output.push_str("Integer"),
                    JsonType::Boolean => output.push_str("Boolean"),
                };

                write!(f, "{}", output)
            }
            JsonComponent::Key { name, value } => {
                let mut output: String = String::new();

                // for _ in 0..outer_nested-1 {
                //     output.push_str("\t");
                // }

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