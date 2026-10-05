// Let's continue on finding the sum of subarrays. This time given a positive integer array nums,
// we want to find the length of the shortest subarray such that the subarray sum is at least target.
// Recall the same example with input nums = [1, 4, 1, 7, 3, 0, 2, 5] and target = 10, then
// the smallest window with the sum >= 10 is [7, 3] with length 2. So the output is 2.

// We'll assume for this problem that it's guaranteed target will not exceed the sum of all elements in nums.

// for lenght of array
// add element to window
// while sum_of_window >= target
// record min len
// remove left element
// shrink window by moving l inward

fn subarray_sum_shortest(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut sum = 0;
    let mut shortest_window = usize::MAX;
    for r in 0..nums.len() {
        sum += nums[r];

        while sum >= target {
            shortest_window = shortest_window.min(r - l + 1);
            sum -= nums[l];
            l += 1;
        }
    }

    if shortest_window as i32 == -1 {
        return 1;
    }

    shortest_window as i32
}

#[cfg(test)]
mod tests {

    use super::*;
    use two_pointers::string_to_vec;

    #[test]
    fn subarray_sum_fixed_works() {
        let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
            (string_to_vec("1 4 1 7 3 0 2 5"), 10, 2),
            (string_to_vec("1 4 1 7 3 10 2 5"), 10, 1),
            (string_to_vec("6 6 6 6 6 6 6"), 19, 4),
            (string_to_vec("1 1 1 "), 3, 3),
            (string_to_vec("100"), 50, 1),
            (string_to_vec("50"), 100, 1),

        ];

        for (input, target, out) in test_cases {
            assert_eq!(subarray_sum_shortest(input, target), out);
        }
    }
}
