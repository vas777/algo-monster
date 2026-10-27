// Subsets
// Prereq: Combination sum

// Given a set of distinct integers, nums, return all possible subsets (the power set).

// Note: The solution set must not contain duplicate subsets.

// Example 1:
// Input: nums = [1,2,3]
// Output:

// Copy
// [
//   [3],
//   [1],
//   [2],
//   [1,2,3],
//   [1,3],
//   [2,3],
//   [1,2],
//   []
// ]


fn dfs(start_index: usize, nums: &[i32], path: &mut Vec<i32> , res: &mut Vec<Vec<i32>> ) {
    res.push(path.clone());
   
    for i in start_index..nums.len() {
        path.push(nums[i]);
        dfs(i+1, nums, path, res);
        path.pop();
    }

}


fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::<Vec<i32>>::new();
    let mut path = Vec::new();
    dfs(0, &nums, &mut path, &mut res);
    res
}