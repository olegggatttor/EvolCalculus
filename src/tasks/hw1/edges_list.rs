use crate::tasks::hw1::graph::Graph;
use std::collections::VecDeque;

use super::adjacency_list::AdjacencyList;

#[derive(Clone)]
pub struct EdgesList {
    graph: Vec<(usize, usize)>,
    graph_size: usize
}

pub struct EdgesListIntoIterator {
    edges_list: EdgesList,
    index: usize
}

impl Iterator for EdgesListIntoIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.edges_list.size() {
            let to_ret = self.index;
            self.index += 1;
            Some(self.edges_list.graph[to_ret])
        } else {
            None
        }
    }
}

impl IntoIterator for EdgesList {
    type IntoIter = EdgesListIntoIterator;
    type Item = (usize, usize);

    fn into_iter(self) -> Self::IntoIter {
        EdgesListIntoIterator {
            edges_list: self,
            index: 0
        }
    }
}

impl Graph for EdgesList {
    fn new(graph_size: usize) -> Self {
        Self {
            graph: Vec::new(),
            graph_size: graph_size
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.graph.push((from, to))
    }

    fn size(&self) -> usize {
        self.graph_size
    }

    fn bfs(&self, source: usize) -> Vec<usize> {
        let mut destinations = vec![usize::MAX; self.size()];
        destinations[source] = 0;
        let mut queue = VecDeque::from([source]);
        while let Some(from) = queue.pop_front() {
            for &(from_graph, to) in &self.graph  {
                if from_graph != from {
                    continue
                }
                if destinations[to] == usize::MAX {
                    destinations[to] = destinations[from] + 1;
                    queue.push_back(to);
                }
            }
        }
        destinations
    }
}

impl From<AdjacencyList> for EdgesList {
    fn from(value: AdjacencyList) -> Self {
        value.into()
    }
}