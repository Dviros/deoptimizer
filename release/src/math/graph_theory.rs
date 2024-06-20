
use petgraph::Graph;

pub struct GraphTheory;

impl GraphTheory {
    pub fn new() -> Self {
        GraphTheory
    }

    pub fn analyze(&self, instructions: &[u8]) -> Graph<u8, ()> {
        let mut graph = Graph::<u8, ()>::new();
        for instruction in instructions {
            graph.add_node(*instruction);
        }
        graph
    }
}
