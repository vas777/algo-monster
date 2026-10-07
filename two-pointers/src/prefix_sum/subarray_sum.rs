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
    let mut prefix_sum_map = HashMap::<i32, i32>::new();
    prefix_sum_map.insert(0, 0);
    let mut ans = Vec::new();
    let mut curr_sum = 0;

    for i in 0..arr.len() {
        curr_sum += arr[i];
        let complement = curr_sum - target;
        if prefix_sum_map.contains_key(&complement) {
            let j = prefix_sum_map.get(&complement).expect("must be in");
            ans.push(*j);
            ans.push((i + 1) as i32);
            return ans;
        }
        prefix_sum_map.insert(curr_sum, (i + 1) as i32);
    }

    Vec::new()
}

/// THE NOT PRETTY
fn subarray_sum_not_pretty(arr: Vec<i32>, target: i32) -> Vec<i32> {
    // build prefix sum array
    // build has map with k = sum ; v = index in array
    let mut prefix_sum = Vec::<i32>::new();
    let mut prefix_sum_map = HashMap::<i32, Vec<i32>>::new();
    let mut ans = Vec::new();
    prefix_sum.push(0);

    for (i, v) in arr.iter().enumerate() {
        prefix_sum.insert(i + 1, prefix_sum[i] + v);
    }

    for (i, v) in prefix_sum.iter().enumerate() {
        // needs vector because the same sum could be at different locations
        prefix_sum_map
            .entry(*v)
            .or_insert(Vec::new())
            .push(i as i32);
    }

    let mut it = prefix_sum.iter().enumerate();
    it.next();

    for (i, _) in it {
        let complement = prefix_sum[i] - target;
        // TODO check and get -> feels like there should be better way
        if prefix_sum_map.contains_key(&complement) {
            let ci = prefix_sum_map
                .get(&complement)
                .expect("this must be in we just checked");
            // find the smallest index among all recorded for this sum
            let Some(pos) = ci.iter().position(|e| *e < i as i32) else {
                continue;
            };

            let Some(min) = ci.get(pos) else {
                continue;
            };

            ans.push(*min);
            ans.push(i as i32);
            break;
        }
    }

    ans
}

#[cfg(test)]
mod tests {

    use super::*;
    use two_pointers::string_to_vec;

    #[test]
    fn subarray_sum_works() {
        let test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
            (string_to_vec("1 -20 -3 30 5 4"), 7, string_to_vec("1 4")),
            (string_to_vec("10 -5 3 2"), 0, string_to_vec("1 4")),
            (string_to_vec("0 0 0"), 0, string_to_vec("0 1")),
            (string_to_vec("1 -1 1 -1 1 -1"), 0, string_to_vec("0 2")),
        ];

        // for (input, w, out) in test_cases.clone() {
        // assert_eq!(subarray_sum(input, w), out);
        // }

        for (input, w, out) in test_cases {
            assert_eq!(subarray_sum_not_pretty(input, w), out);
        }
    }
}
