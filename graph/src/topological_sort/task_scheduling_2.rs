// This problem extends Task Scheduling. Each task now has a duration, where times[i] is how long tasks[i] takes.

// You can run any number of tasks in parallel, as long as dependencies are respected. If [a, b] appears in requirements, task a must finish before task b can start.

// Return the minimum total time required to complete all tasks.

// There is guaranteed to be a solution.

// Examples
// Example 1
// Input:

// Copy
// tasks = ["a", "b", "c", "d"]
// times = [1, 1, 2, 1]
// requirements = [["a", "b"], ["c", "b"], ["b", "d"]]
// Output: 4
// Figure
// Example 1 dependency graph with task durations

// The longest dependency chain is c -> b -> d, so the minimum total time is 2 + 1 + 1 = 4.

use std::collections::{HashMap, VecDeque};

fn get_neighbors<'a>(
    tasks: &'a Vec<String>,
    times: &'a Vec<i32>,
    requirements: &'a Vec<Vec<String>>,
) -> (
    HashMap<&'a str, i32>,
    HashMap<&'a str, Vec<&'a str>>,
    HashMap<&'a str, i32>,
) {
    let mut indegree = HashMap::<&str, i32>::new();
    let mut adj = HashMap::<&str, Vec<&str>>::new();
    let mut duration = HashMap::<&str, i32>::new();

    for (task, time) in tasks.iter().zip(times.iter()) {
        indegree.entry(task).or_default();
        duration.entry(task).or_insert(*time);
    }

    for req in requirements.iter() {
        let [prer, dep] = req.as_slice() else {
            continue;
        };

        adj.entry(prer).or_default().push(dep);
        indegree.entry(dep).and_modify(|x| *x += 1);
    }

    (indegree, adj, duration)
}

fn task_scheduling_2(tasks: Vec<String>, times: Vec<i32>, requirements: Vec<Vec<String>>) -> i32 {
    // let tasks = tasks.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let mut order = VecDeque::<&str>::new();
    let mut finish_time = HashMap::<&str, i32>::new();
    let (mut indegrees, adj, duration) = get_neighbors(&tasks, &times, &requirements);
    let mut total = 0;
    finish_time = duration.clone();

    for (k, v) in indegrees.iter() {
        if *v == 0 {
            order.push_back(k);
        }
    }

    while let Some(node) = order.pop_front() {
        let d = duration[node];
        total += d;
        // finish_time[node] =
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

    total
}
