// Clone Graph
// You are given a connected undirected graph and must return a deep copy (clone)
// of it — a new graph with the same nodes and the same edges, built independently of the input.

// The n nodes are labeled 1 through n, and adjList holds one row per node:
// row 0 lists the neighbors of node 1, row 1 lists the neighbors of node 2,
// and so on. The numbers inside a row are node labels, not row indices,
// so they run from 1 to n. Because the graph is undirected the rows are
// symmetric: if b appears in node a's row, then a appears in node b's row.

// Return the clone in the same adjacency-list form.

// Input & Output
// Input
// adj_list — An adjacency list of the graph: row i (0-indexed) holds the labels of the neighbors of node i + 1, each label in the range 1..n.
// Output
// A deep copy of the graph represented as an adjacency list with the same structure as the input.
// Example
// Input
// adjList = [[2,4],[1,3],[2,4],[1,3]]
// Output
// [[2,4],[1,3],[2,4],[1,3]]
// Constraints
// The number of nodes n is in the range [0, 100]
// Node labels are 1 through n, one row of adjList per node
// There are no repeated edges and no self-loops in the graph
// The Graph is connected and all nodes can be visited starting from the given node

use std::collections::HashSet;

fn clone_graph_trivial(adj_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // WRITE YOUR BRILLIANT CODE HERE
    adj_list.clone()
}

fn dfs(node: i32, adj_list: &[Vec<i32>], visited: &mut HashSet<i32>, res: &mut [Vec<i32>]) {
    visited.insert(node);
    let idx = (node - 1) as usize;

    for &neighbor in &adj_list[idx] {
        res[idx].push(neighbor);
        if !visited.contains(&neighbor) {
            dfs(neighbor, adj_list, visited, res);
        }
    }
}

fn clone_graph(adj_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if adj_list.is_empty() {
        return Vec::new();
    }

    let n = adj_list.len();
    let mut res = vec![Vec::new(); n];
    let mut visited = HashSet::new();

    dfs(1, &adj_list, &mut visited, &mut res);
    res
}
