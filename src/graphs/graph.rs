use std::collections::{HashMap,HashSet};

#[derive(Debug)]
pub struct Graph<T> where T: std::cmp::Eq + std::hash::Hash + std::clone::Clone, {
    adjacency_list: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T> where T: std::cmp::Eq + std::hash::Hash + std::clone::Clone, {
    pub fn new() -> Graph<T> {
        Graph { adjacency_list: HashMap::new() }
    }

    pub fn add_vertex(&mut self, vertex: T) {
        self.adjacency_list.insert(vertex, HashSet::new());
    }

    pub fn add_edge(&mut self, source: T, destination: T) {
        self.adjacency_list.get_mut(&source).unwrap().insert(destination.clone());
        self.adjacency_list.get_mut(&destination).unwrap().insert(source);
    }

    pub fn add_directed_edge(&mut self, source: T, destination: T) {
        self.adjacency_list.get_mut(&source).unwrap().insert(destination);
    }

    pub fn get_adjacent_vertices(&self, vertex: T) -> &HashSet<T> {
        self.adjacency_list.get(&vertex).unwrap()
    }
}
