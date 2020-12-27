//! A graph data structure backed by a Vec and a Hashmap. The Vec stores the
//! vertices whereas the Hashmap provides a quick way to find a vertex by its
//! name.

// todo:
// - implement tests
// - test performance for this implementation agains one soleny based on HashMap

use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Graph {
    names: HashMap<String, usize>,
    vertices: Vec<Vertex>,
}

struct Vertex {
    name: String,
    pointed_by: Vec<Edge>,
    points_to: Vec<Edge>,
}

struct Edge {
    to_vertex: usize,
    weight: u32,
}

//--------------------------------------------------------------------
// Implementation
//--------------------------------------------------------------------

impl Graph {
    pub fn new() -> Self {
        Graph::default()
    }

    pub fn add_edge(&mut self, vertex0: &str, vertex1: &str, weight: u32) {
        let v0 = self.get_or_add_vertex(vertex0);
        let v1 = self.get_or_add_vertex(vertex1);
        self.vertices[v0].points_to.push(Edge::new(v1, weight));
        self.vertices[v1].pointed_by.push(Edge::new(v0, weight));
    }

    pub fn add_edges(&mut self, vertex0: &str, vertices: &[(u32, String)]) {
        for vertex in vertices {
            self.add_edge(vertex0, &vertex.1, vertex.0);
        }
    }

    /// Returns a set containing the indexes of all ancestor vertices
    pub fn list_ancestors(&self, name: &str) -> HashSet<usize> {
        let mut cache = HashSet::<usize>::new();
        if let Some(&idx) = self.names.get(name) {
            self.rec_list_ancestors(&mut cache, idx);
        }
        cache
    }

    /// Returns the combined weight of all successors vertices
    /// For example, the graph 1 - 2 - 3
    ///                             |- 4
    /// has a combined weight of 1 + 1 * ( 2 + 2 * (4 + 3)) = 17.
    pub fn weigh_successors(&self, name: &str) -> u32 {
        let mut weight = 0;
        if let Some(&idx) = self.names.get(name) {
            weight = self.rec_weigh_sucessors(idx);
        }
        weight
    }

    //------------------------------
    // Helpers
    //------------------------------

    fn rec_list_ancestors(&self, cache: &mut HashSet<usize>, idx: usize) {
        let pointed_by = &self.vertices[idx].pointed_by;
        for edge in pointed_by {
            if !cache.contains(&edge.to_vertex) {
                cache.insert(edge.to_vertex);
                self.rec_list_ancestors(cache, edge.to_vertex);
            }
        }
    }

    fn rec_weigh_sucessors(&self, idx: usize) -> u32 {
        let points_to = &self.vertices[idx].points_to;
        let mut weight = 0;
        for edge in points_to {
            weight += edge.weight + edge.weight * self.rec_weigh_sucessors(edge.to_vertex);
        }
        weight
    }

    fn add_vertex(&mut self, v: Vertex) -> usize {
        let idx = self.vertices.len();
        self.vertices.push(v);
        self.names
            .insert(self.vertices.last().unwrap().name.clone(), idx);
        idx
    }

    fn get_or_add_vertex(&mut self, vertex: &str) -> usize {
        let idx;
        if !self.names.contains_key(vertex) {
            idx = self.add_vertex(Vertex::new(vertex));
        } else {
            idx = *self.names.get(vertex).unwrap();
        }
        idx
    }
}

impl Vertex {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            pointed_by: Vec::new(),
            points_to: Vec::new(),
        }
    }
}

impl Edge {
    fn new(to_vertex: usize, weight: u32) -> Self {
        Self { to_vertex, weight }
    }
}
//--------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------

#[cfg(test)]
mod tests {}
