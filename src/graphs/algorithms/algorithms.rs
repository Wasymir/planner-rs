trait StepHandler {
    fn handle(&self, graph: &Graph, vertex: u32, elements: &mut Vec<Box<Any>>);
}
