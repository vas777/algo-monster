// Reconstructing Sequence
// Prereq: Topological Sort

// A sequence s is a list of integers. Its subsequence is a new sequence that can be made up by deleting elements from s, without changing the order of integers.

// We are given an original sequence (which is a permutation of the integers from 1 to n) and a list of subsequences seqs.

// Determine whether original is the only sequence that can be reconstructed from seqs. Reconstruction means building the shortest sequence so that all sequences in seqs are subsequences of it.

// Parameters
// original: a list of integers of size n representing the original sequence.
// seqs: a list of sequences of size m representing the sequences to be reconstructed.
// Result
// true or false, depending on whether the original sequence can be uniquely reconstructed.
// Examples
// Example 1:
// Input: original: [1,2,3], seqs: [[1,2], [1,3]]

// Output: false

// Explanation:

// [1,2,3] is not the only one sequence that can be reconstructed, because [1,3,2] is also a valid sequence that can be reconstructed.

// Example 2:
// Input: original: [1,2,3], seqs: [[1,2]]

// Output: false

// Explanation:

// There is only one subsequence, so the reconstructed original sequence can only be [1,2] which is missing 3.

// Example 3:
// Input: orginal: [1,2,3], seqs: [[1,2], [1,3], [2,3]]

// Output: true

// Explanation:

// [1,2,3] is the only sequence that can be reconstructed from [1,2], [1,3], and [2,3].

// Example 4:
// Input: original: [4,1,5,2,6,3], seqs: [[5,2,6,3], [4,1,5,2]]

// Output: true

// Explanation:

// [4,1,5,2,6,3] is the only sequence that can be reconstructed from [[5,2,6,3], [4,1,5,2]].

// Constraints
// 1 <= n <= 10^4
// 1 <= m <= 10^4
// 1 <= len(seqs[i]) <= n

use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Add,
};

fn sequence_reconstruction(original: Vec<i32>, seqs: Vec<Vec<i32>>) -> bool {
    let mut neighbors = HashMap::<i32, HashSet<i32>>::new();
    let mut order = VecDeque::<i32>::new();
    let mut indegree = HashMap::<i32, i32>::new();
    let mut res = Vec::<i32>::new();
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for o in original.iter() {
        min = min.min(*o);
        max = max.max(*o);
        indegree.insert(*o, 0);
    }

    // process seqs with windows(2) to produce
    // neighbors list
    for s in seqs {
        if s.iter().any(|&x| x < min || x > max) {
            return false;
        }
        let iter = s.windows(2);
        for w in iter {
            if neighbors.entry(w[0]).or_default().insert(w[1]) {
                indegree.entry(w[1]).and_modify(|x| {
                    *x += 1;
                });
            }
        }
    }

    for (k, v) in indegree.iter() {
        // sequence might have elements outside original
        // range
        if *v == 0 {
            order.push_back(*k);
        }
    }

    if order.len() > 1 {
        return false;
    }

    while let Some(element) = order.pop_front() {
        res.push(element);
        if let Some(ng_list) = neighbors.get(&element) {
            for ng in ng_list {
                if let Some(ng_indegree) = indegree.get_mut(ng) {
                    *ng_indegree -= 1;
                    if *ng_indegree == 0 {
                        order.push_back(*ng);
                        if order.len() > 1 {
                            return false;
                        }
                    }
                }
            }
        }
    }

    res == original
}
