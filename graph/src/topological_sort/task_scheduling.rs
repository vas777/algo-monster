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

fn build_indegree<'a>(tasks: &Vec<String>, requirements: &'a Vec<Vec<String>>) -> HashMap<&'a String, (i32, Vec<&'a String>)> {
    let mut res = HashMap::<&String, (i32, Vec<&String>)>::new();

    for t in tasks {
        res.entry(t).or_default();
    }

    for node in requirements {
        // second elements becomes neighbours
        // or in other words comes after first
        // so increase its in-degree
        let current_node = node.iter().nth(0).unwrap();
        let mut curr_node_entry = res.get_mut(current_node).unwrap();

        for neighbours in node.iter().skip(0) {
            // add to neighbours of current node
            curr_node_entry.1.push(neighbours);
            // increment indegree of neighour
            if let Some(n_v) = res.get_mut(neighbours) {
                n_v.0 += 1;

            }
        }
    }
    res
}

fn task_scheduling(tasks: Vec<String>, requirements: Vec<Vec<String>>) -> Vec<String> {
    let mut res = Vec::<String>::with_capacity(tasks.len());
    let mut order = VecDeque::<&String>::new();
    let indegrees = build_indegree(&requirements);

    for (k, v) in indegrees {
        if v == 0 {
            order.push_back(k);
        }
    }

    while !order.is_empty() {
        if let Some(node) = order.pop_front() {}
    }

    Vec::new()
}
