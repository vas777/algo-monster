// Given an (unweighted) connected graph, return the length of the shortest
// path between two nodes A and B, in terms of the number of edges.

// Assume there always exists a path between nodes A and B.

// Input:

// Copy
// graph = [[1, 2], [0, 2, 3], [0, 1], [1]]
// A = 0
// B = 3
// Output: 2

// Note that the graph input is an adjacency list representation, not an adjacency matrix or a 2D grid. Specifically,
// graph[i] is the list of neighbor nodes for node i. For example, graph = [[1, 2], [0], [0]] means node 0
// has edges to nodes 1 and 2. You can use the BFS template from BFS on Graphs as your starter code.

//     0(A)
//    / \
//   1 - 2
//  /
// 3(B)

// USE one of the nodes
// grap[A]
// locate one of the nodes
// one for sure will be before other
// start bfs from it
// bfs gives us shortest path.
// bfs
// start counting

use std::collections::{HashSet, VecDeque};
fn shortest_path(graph: Vec<Vec<i32>>, a: i32, b: i32) -> i32 {
    let mut order = VecDeque::<i32>::new();
    let mut visited = HashSet::<i32>::new();
    let mut level = 0;
    let mut node_per_level = 1;
    order.push_back(a);
    visited.insert(a);
    while !order.is_empty() {
        for _ in 0..node_per_level {
            let vertex = order.pop_front().unwrap();
            if vertex == b {
                return level;
            }
            for v in graph[vertex as usize].iter() {
                if visited.contains(&v) {
                    continue;
                }
                order.push_back(*v);
                visited.insert(*v);
            }
        }
        node_per_level = order.len();
        level += 1;
    }

    level
}
