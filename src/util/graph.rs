use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash)]
struct Edge<Vertex> {
    from: Vertex,
    to: Vertex,
}

pub struct Graph<Vertex> {
    vertices: HashSet<Vertex>,
    edges: HashSet<Edge<Vertex>>,
}

impl<Vertex: Clone + PartialEq + Eq + Hash> Graph<Vertex> {
    pub fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    pub fn add_vertex(&mut self, v: Vertex) -> bool {
        self.vertices.insert(v)
    }

    pub fn add_edge(&mut self, from: Vertex, to: Vertex) {
        self.edges.insert(Edge { from, to });
    }

    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    pub fn shortest_path(&self, from: Vertex, to: Vertex) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut distances = HashMap::new();

        distances.insert(from, 0);

        loop {
            let current_node = self
                .vertices
                .iter()
                .filter(|node| distances.contains_key(*node))
                .filter(|node| !visited.contains(*node))
                .min_by_key(|node| distances[*node])?;

            let distance = distances[current_node];

            if *current_node == to {
                return Some(distance);
            }

            visited.insert(current_node);

            for edge in self.edges.iter() {
                if edge.from == *current_node {
                    let entry = distances.entry(edge.to.clone()).or_insert(distance + 1);

                    if *entry > distance + 1 {
                        *entry = distance + 1;
                    }
                }
            }
        }
    }
}
