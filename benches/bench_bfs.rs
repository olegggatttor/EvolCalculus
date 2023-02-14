use glassbench::*;
use evol_calculus::tasks::hw1::{graph::Graph, adjacency_list::AdjacencyList, edges_list::EdgesList};

const BENCH_GRAPH_SIZE: usize = 1000;
const BENCH_EDGES_AMOUNT: usize = 99000;
const SOURCE: usize = 0;


fn build_graph<T: Graph>() -> T {
    let mut graph = T::new(BENCH_GRAPH_SIZE);
    for i in 0..BENCH_EDGES_AMOUNT {
        graph.add_edge(i % BENCH_GRAPH_SIZE, i / BENCH_GRAPH_SIZE + 1)
    }
    graph
}

fn bench_bfs(bench: &mut Bench) {
    bench.task("BFS using list of edges", |task| {
        let graph = Box::new(build_graph::<EdgesList>());
        task.iter(|| {
            pretend_used(graph.bfs(SOURCE));
        })
    });
    bench.task("BFS using adjacency list", |task| {
        let graph = build_graph::<AdjacencyList>();
        task.iter(|| {
            pretend_used(graph.bfs(SOURCE));
        })
    });
}

glassbench!(
    "HW1: BFS",
    bench_bfs,
);