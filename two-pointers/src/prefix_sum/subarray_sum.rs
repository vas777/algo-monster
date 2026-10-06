
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
    // build prefix sum array
    // build has map with k = sum ; v = index in array

    // i < j
    // sum [i,j) = sum[0,j) - sum[0,i)
    // when I am at prefix_sum[j]
    // is there prefix_sum[i]
    // subtraction gets target  
    // is (arr[j] - target) in prefix sum


    Vec::new()
}

#[cfg(test)]
mod tests {

    use super::*;
    use two_pointers::string_to_vec;

    #[test]
    fn subarray_sum_works() {
        let test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
            (string_to_vec("1 -20 -3 30 5 4"), 7, string_to_vec("1 4")),
        ];

        for (input, w, out) in test_cases {
            assert_eq!(subarray_sum(input, w), out);
        }
    }
}