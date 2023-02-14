pub trait Graph {
    fn new(graph_size: usize) -> Self;

    fn add_edge(&mut self, from: usize, to: usize);

    fn bfs(&self, source: usize) -> Vec<usize>;

    fn size(&self) -> usize;
}