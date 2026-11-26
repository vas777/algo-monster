// There are a total of n courses a student has to take, numbered from 0 to n-1. A course may have prerequisites. The "depends on" relationship is expressed as a pair of numbers. For example, [0, 1] means you need to take course 1 before taking course 0. Given n and the list of prerequisites, decide if it is possible to take all the courses.

// Example 1:

// Input: n = 2, prerequisites = [[0, 1]]

// Output: true

// Explanation: we can take 1 first and then 0.

// Example 2:

// Input: n = 2, prerequisites = [[0, 1], [1, 0]]

// Output: false

// Explanation: the two courses depend on each other, there is no way to take them

use std::collections::VecDeque;

fn build_indegree(n: i32, prerequisites: &[Vec<i32>]) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut indegree = vec![0; n as usize];
    let mut adj = vec![vec![]; n as usize];

    for node in prerequisites {
        let [course, prereq] = node.as_slice() else {
            continue;
        };

        indegree[*course as usize] += 1;
        adj[*prereq as usize].push(*course);
    }
    (indegree, adj)
}

fn is_valid_course_schedule(n: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut res = Vec::<i32>::new();
    let mut order = VecDeque::<i32>::with_capacity(n as usize);
    let (mut indegrees, adj) = build_indegree(n, &prerequisites);

    for (i, val) in indegrees.iter().enumerate() {
        if *val == 0 {
            order.push_back(i as i32);
        }
    }

    while let Some(node) = order.pop_front() {
        res.push(node);
        let neighbors = &adj[node as usize];
        for neighbor in neighbors.iter() {
            indegrees[*neighbor as usize] -= 1;
            if indegrees[*neighbor as usize] == 0 {
                order.push_back(*neighbor);
            }
        }
    }

    res.len() as i32 == n
}
