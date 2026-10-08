// Given an integer array nums, calculate the sum of elements between indices left and right (inclusive). You need to answer multiple queries efficiently. You are required to preprocess the array so that each query can be answered in constant time.

// Example: Input: nums = [1, 2, 3, 4], sumRange(1, 3). Output: 9.

// Your function should return 9 because the sum of elements from index 1 to 3 is 2 + 3 + 4 = 9.

fn range_sum_query_immutable(nums: Vec<i32>, left: i32, right: i32) -> i32 {
    // general idea would be to precompute this
    let mut prefix_sum_arr = Vec::new();
    prefix_sum_arr.push(0);
    let mut curr_sum = 0;
    for i in 0..nums.len() {
        curr_sum += nums[i];
        prefix_sum_arr.insert(i + 1, curr_sum as i32);
    }

    let res = prefix_sum_arr[(right + 1) as usize] - prefix_sum_arr[left as usize];

    res
}
