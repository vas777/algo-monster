// Recall finding the largest size k subarray sum of an integer array in Largest Subarray Sum.
// What if we don't need the largest sum among all subarrays of fixed size k, but instead,
// we want to find the length of the longest subarray with sum smaller than or equal to a target?
//
// Given an array of non-negative integers nums = [1, 6, 3, 1, 2, 4, 5] and target = 10,
// the longest subarray that does not exceed 10 is [3, 1, 2, 4], so the output is 4.

// for r smaller than len of arr.len
// while sum of windows is > targe
// shrink window
// evaluate window len

use std::cmp::max;

fn subarray_sum_longest(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut window_sum = 0;
    let mut max_window_len = 0;

    for r in 0..nums.len() {
        window_sum += nums[r];

        while window_sum > target {
            window_sum = window_sum - nums[l];
            l += 1;
        }

        if r >= l && r - l + 1 > max_window_len {
            max_window_len = r - l + 1;
        }
    }

    max_window_len as i32
}

#[cfg(test)]
mod tests {

    use super::*;
    use two_pointers::string_to_vec;

    #[test]
    fn subarray_sum_longest_works() {
        let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
            (string_to_vec("1 6 3 1 2 4 5"), 10, 4),
            (string_to_vec("1 6 3 1 10 4 5"), 10, 3),
            (string_to_vec("1 4 2 10 23 3 1 0 20"), 8, 3),
            (string_to_vec("5 2 1 0 3"), 4, 3),
            (string_to_vec("2 1 3 4 1"), 1, 1),
            (string_to_vec("1 2 3 4 5 6 7 8 9"), 15, 5),
            (string_to_vec("1 2 3 4 5"), 15, 5),
            (string_to_vec("1 2 3 4 5"), 16, 5),
        ];

        for (input, target, out) in test_cases {
            assert_eq!(subarray_sum_longest(input, target), out);
        }
    }
}
