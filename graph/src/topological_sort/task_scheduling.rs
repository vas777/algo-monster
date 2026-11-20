// For this problem, given a list of tasks and a list of requirements, compute a sequence of tasks that can be performed, such that we complete every task once while satisfying all the requirements.

// Each requirement will be in the form of a list [a, b], where task a needs to be completed first before task b can be completed,

// There is guaranteed to be a solution.

// Examples
// Example 1
// Input:

// Copy
// tasks = ["a", "b", "c", "d"]
// requirements = [["a", "b"], ["c", "b"], ["b", "d"]]
// Output: ["a", "c", "b", "d"]

use std::collections::{HashMap, VecDeque};

fn build_indegree<'a, 'b: 'a>(
    tasks: &'b Vec<String>,
    requirements: &'a Vec<Vec<String>>,
) -> (HashMap<&'a str, i32>, HashMap<&'a str, Vec<&'a str>>) {
    let mut indegree = HashMap::<&str, i32>::new();
    let mut adj = HashMap::<&str, Vec<&str>>::new();

    // if node is not in neighbour
    // it won't be inserted unless this loop
    for t in tasks {
        indegree.entry(t).or_default();
    }

    for node in requirements {
        let [current_node, neighbor] = node.as_slice() else {
            continue;
        };

        adj.entry(current_node).or_default().push(neighbor);
        *indegree.entry(neighbor).or_insert(0) += 1;
    }
    (indegree, adj)
}

fn task_scheduling(tasks: Vec<String>, requirements: Vec<Vec<String>>) -> Vec<String> {
    let mut res = Vec::<&str>::new();
    let mut order = VecDeque::<&str>::new();
    let (mut indegrees, adj) = build_indegree(&tasks, &requirements);

    for (k, v) in indegrees.iter() {
        if *v == 0 {
            order.push_back(k);
        }
    }

    while let Some(node) = order.pop_front() {
        res.push(node);
        if let Some(neighbors) = adj.get(node) {
            for &neighbor in neighbors {
                if let Some(value) = indegrees.get_mut(neighbor) {
                    *value -= 1;
                    if *value == 0 {
                        order.push_back(neighbor);
                    }
                }
            }
        }
    }

    if res.len() == tasks.len() {
        res.into_iter().map(|s| s.to_owned()).collect()
    } else {
        Vec::new()
    }
}
