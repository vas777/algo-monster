// This problem applies the prefix sum technique from the introduction:
// instead of testing every possible subarray, a running prefix sum turns
// the search into a hash table lookup.

// Given an integer array arr and a target value, return a subarray whose
// sum equals the target. Return the answer as [start, end),
// where start is inclusive and end is exclusive. If there are multiple valid
// answers, return the one with the smaller end value.

// Input: arr = [1, -20, -3, 30, 5, 4], target = 7

// Output: [1, 4]

// The subarray arr[1:4] = [-20, -3, 30] sums to 7.

use std::collections::HashMap;

fn subarray_sum(arr: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res = Vec::with_capacity(2);
    // complement => index
    // have we seen this value? => where is it?
    let mut prefix_sum = HashMap::<i32, i32>::new();
    prefix_sum.insert(0, 0);
    let mut curr_sum = 0;
    for (i, v) in arr.iter().enumerate() {
        curr_sum += v;
        let complement = curr_sum - target;
        if let Some(&ci) = prefix_sum.get(&complement) {
            res.push(ci);
            res.push((i + 1) as i32);
            break;
        }
        prefix_sum.insert(curr_sum, (i + 1) as i32);
    }

    res
}
