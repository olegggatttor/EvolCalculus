use crate::tasks::hw1::graph::Graph;
use std::collections::VecDeque;

use super::edges_list::EdgesList;

#[derive(Clone)]
pub struct AdjacencyList {
    graph: Vec<Vec<usize>>
}

pub struct AdjacencyListIntoIterator {
    adjacency_list: AdjacencyList,
    pos: (usize, usize),
    count: usize
}

impl Iterator for AdjacencyListIntoIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.adjacency_list.size() {
            let row = &self.adjacency_list.graph[self.pos.0];
            let result = (self.pos.0, row[self.pos.1]);
            let new_pos = if self.pos.1 + 1 >= row.len() {
                (self.pos.0 + 1, 0)
            } else {
                (self.pos.0, self.pos.1 + 1)
            };
            self.pos = new_pos;
            self.count += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl IntoIterator for AdjacencyList {
    type IntoIter = AdjacencyListIntoIterator;
    type Item = (usize, usize);

    fn into_iter(self) -> Self::IntoIter {
        AdjacencyListIntoIterator {
            adjacency_list: self,
            pos: (0, 0),
            count: 0
        }
    }
}

impl Graph for AdjacencyList {
    fn new(graph_size: usize) -> Self {
        Self {
            graph: vec![Vec::new(); graph_size]
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.graph[from].push(to)
    }

    fn size(&self) -> usize {
        self.graph.len()
    }

    fn bfs(&self, source: usize) -> Vec<usize> {
        let mut destinations = vec![usize::MAX; self.size()];
        destinations[source] = 0;
        let mut queue = VecDeque::from([source]);
        while let Some(from) = queue.pop_front() {
            for &to in &self.graph[from]  {
                if destinations[to] == usize::MAX {
                    destinations[to] = destinations[from] + 1;
                    queue.push_back(to);
                }
            }
        }
        destinations
    }
}

impl From<EdgesList> for AdjacencyList {
    fn from(edges_list: EdgesList) -> AdjacencyList {
        let mut graph = AdjacencyList::new(edges_list.size());
        for edge in edges_list {
            graph.add_edge(edge.0, edge.1);
        }
        graph
    }
}