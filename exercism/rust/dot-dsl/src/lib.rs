pub mod graph {

    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            return Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            };
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            for node in nodes {
                self.nodes.push(node.clone());
            }
            return self;
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            for edge in edges {
                self.edges.push(edge.clone());
            }
            return self;
        }

        // attrs: &[(&str, &str); 1]
        // attrs: &Vec<(String, String)>
        // attrs: &Vec<(&str, &str)>
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for &(key, val) in attrs {
                self.attrs.insert(key.to_string(), val.to_string());
            }
            return self;
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            return self.nodes.iter().find(|n| n.name == name);
        }
    }

    pub mod graph_items {

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    return Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    };
                }

                // attrs: &[(&str, &str); 1]
                // attrs: &Vec<(String, String)>
                // attrs: &Vec<(&str, &str)>
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for &(key, val) in attrs {
                        self.attrs.insert(key.to_string(), val.to_string());
                    }
                    return self;
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    return Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    };
                }

                // attrs: &[(&str, &str); 1]
                // attrs: &Vec<(String, String)>
                // attrs: &Vec<(&str, &str)>
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for &(name, value) in attrs {
                        self.attrs.insert(name.to_string(), value.to_string());
                    }
                    return self;
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    return match self.attrs.get(name) {
                        Some(v) => {
                            // v is a &std::string::String
                            return Some(v.as_str());
                        },
                        None => None,
                    };
                }
            }
        }
    }
}
