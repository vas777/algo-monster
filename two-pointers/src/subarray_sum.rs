// Fixed Size Sliding Window
// Given an array (list) nums consisted of only non-negative integers, find the largest sum among all subarrays of length k in nums.

// For example, if the input is nums = [1, 2, 3, 7, 4, 1], k = 3, then the output would be 14 as the largest length 3 subarray sum is given by [3, 7, 4] which sums to 14.

fn subarray_sum_fixed(nums: Vec<i32>, k: i32) -> i32 {
    let mut l = 0;
    let mut max = 0;
    let mut curr_val = 0;

    for r in 0..k {
        curr_val += nums[r as usize];
    }
    max = curr_val;

    for r in k as usize..nums.len() {
        curr_val = curr_val - nums[l] + nums[r];
        if curr_val > max {
            max = curr_val;
        }
        l += 1;
    }

    max
}

#[cfg(test)]
mod tests {

    use super::*;
    use two_pointers::string_to_vec;

    #[test]
    fn subarray_sum_fixed_works() {
        let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
            (string_to_vec("1 2 3 7 4 1"), 3, 14),
            (string_to_vec("1 2 3 4 5 6"), 2, 11),
            (string_to_vec("6 2 8 1 5"), 2, 10),
            (string_to_vec("6 2 8 1 5"), 1, 8),
            (string_to_vec("1 4 2 10 23 3 1 0 20"), 4, 39),
        ];

        for (input, w, out) in test_cases {
            assert_eq!(subarray_sum_fixed(input, w), out);
        }
    }
}
