use std::collections::{HashSet, VecDeque};

pub struct BFS<'a> {
    avoid_vertices: HashSet<u32>,
    max_steps: isize,
    current_steps: usize,
    stopped_at: Option<u32>,
    handler: &'a dyn StepHandler,
}

impl<'a> BFS<'a> {
    fn new(avoid_vertices: HashSet<u32>, max_steps: isize, handler: &'a dyn StepHandler) -> Self {
        BFS {
            avoid_vertices,
            max_steps,
            current_steps: 0,
            stopped_at: None,
            handler,
        }
    }

    fn new_unlimited(avoid_vertices: HashSet<u32>, handler: &'a dyn StepHandler) -> Self {
        BFS {
            avoid_vertices,
            max_steps: -1,
            current_steps: 0,
            stopped_at: None,
            handler,
        }
    }

    fn get_stopped_at(&self) -> Option<u32> {
        self.stopped_at
    }

    fn run(&mut self, graph: &Graph, start_vertex: u32, visited: &mut HashSet<u32>) {
        let mut queue = VecDeque::new();
        queue.push_back((start_vertex,Vec<Box<Any>>::new()));

        while let Some(pair) = queue.pop_front() {
            vertex = pair.0;
            elements = pair.1;
            self.handler.handle(graph, vertex, elements);
            self.current_steps += 1;

            if self.max_steps != -1 && self.current_steps > self.max_steps as usize {
                self.stopped_at = Some(vertex);
                return;
            }

            visited.insert(vertex);

            for adjacent in graph.get_adjacent_vertices(vertex) {
                if !self.avoid_vertices.contains(adjacent) && !visited.contains(adjacent) {
                    queue.push_back((*adjacent,elements));
                }
            }
        }
    }
}