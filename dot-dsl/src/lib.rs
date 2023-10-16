pub mod graph {
    use std::collections::HashMap;
    use self::graph_items::{ node::Node, edge::Edge };

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Graph { 
                nodes: Vec::new(), 
                edges: Vec::new(), 
                attrs: HashMap::new() 
            }
        }
        pub fn with_nodes(mut self, nodes: impl AsRef<[Node]>) -> Self {
            nodes.as_ref().iter().cloned().for_each(|x| self.nodes.push(x));
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs.iter() { 
                self.attrs.insert(k.to_string(), v.to_string()); 
            }
            self
        }
        pub fn with_edges(mut self, edges: impl AsRef<[Edge]>) -> Self {
            edges.as_ref().iter().cloned().for_each(
                |x| self.edges.push(x)
            );
            self
        }
        pub fn node(&self, x: &str) -> Option<Node> {
            self.nodes.iter().find(|s| s.get(x)).map(|n| n.clone())
        }
        pub fn attr(&self, k: &str) -> Option<&str> {
            self.attrs.get(k).map(|s| &**s)
        }
    }
     
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge{
                start: String,
                end: String,
                attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(start: &str, end: &str) -> Edge {
                    Edge {
                        start: String::from(start),
                        end: String::from(end),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs.iter() { 
                        self.attrs.insert(k.to_string(), v.to_string()); 
                    }
                    self
                }
                pub fn attr(&self, k: &str) -> Option<&str> {
                    self.attrs.get(k).map(|s| &**s)
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(n: &str) -> Self {
                    Self { name: String::from(n), attrs: HashMap::new() }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs.iter() { 
                        self.attrs.insert(k.to_string(), v.to_string()); 
                    }
                    self
                }
                pub fn get(&self, name: &str) -> bool {
                    self.name == name
                }
                pub fn attr(&self, k: &str) -> Option<&str> {
                    self.attrs.get(k).map(|s| &**s)
                }                
            }
        }
    }
}
