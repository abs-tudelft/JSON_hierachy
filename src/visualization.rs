use crate::components::JsonComponent;

type Nd<'a> = (usize, &'a str);
type Ed<'a> = (Nd<'a>, Nd<'a>);
struct Graph { nodes: Vec<String>, edges: Vec<(usize,usize)> }


fn update_graph(component: &JsonComponent, parent_id: Option<usize>, graph: &mut Graph) {
    match component {
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
                update_graph(value, Some(id), graph);
            }
        },
        JsonComponent::Object { outer_nested, inner_nested, records } => {
            graph.nodes.push(format!("Object parser\nO: {}, I: {}", outer_nested, inner_nested));
            let id = graph.nodes.len() - 1;

            if let Some(parent_id) = parent_id {
                graph.edges.push((parent_id, id));
            }

            for record in records {
                update_graph(record, Some(id), graph);
            }
        },
        JsonComponent::Key { name, outer_nested, value } => {
            graph.nodes.push(format!("Key filter\nMatch: \"{}\"\nO: {}", name, outer_nested));
            let id = graph.nodes.len() - 1;

            if let Some(parent_id) = parent_id {
                graph.edges.push((parent_id, id));
            }

            if let Some(value) = value {
                update_graph(value, Some(id), graph);
            }
        },
    }
}

pub fn generate(root: &JsonComponent, output_path: &str) {
    // Separate output path into directory and file name
    let (dir, _) = output_path.split_at(output_path.rfind('/').unwrap_or(0));

    // Create the directory if it doesn't exist
    std::fs::create_dir_all(dir).unwrap();

    // Create the file
    let mut file = std::fs::File::create(output_path).unwrap();

    let mut graph = Graph { nodes: Vec::new(), edges: Vec::new() };
    update_graph(root, None, &mut graph);

    dot::render(&graph, &mut file).unwrap()
}


// Implementation of labeling of nodes and edges inside the dot file
impl<'a> dot::Labeller<'a, Nd<'a>, Ed<'a>> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("schema").unwrap() }
    fn node_id(&'a self, n: &Nd<'a>) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n.0)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &Nd<'b>) -> dot::LabelText<'b> {
        let &(i, _) = n;
        dot::LabelText::LabelStr(std::borrow::Cow::Borrowed(self.nodes[i].as_str()))
    }
}

// Implementation of how to traverse the graph
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