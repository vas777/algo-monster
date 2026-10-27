// Deduplication
// The problem asks: given an array nums and a target, find all unique triplets where nums[i] + nums[j] + nums[k] = target and i, j, k are different indices.
// Equal values are allowed in a triplet as long as they come from different positions (for example, [1, 1, 2] is valid only if the input has at least two 1s).
// The word "unique" is critical. If the array contains [1, 1, 2, 3] and the target is 6, the triplet [1, 2, 3] should appear only once in the output,
// even though we can form it using either of the two 1s.

fn two_sum(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
    let mut l = 0;
    let mut r = nums.len().saturating_sub(1);
    let mut res = Vec::<(i32, i32)>::new();
    while l < r {
        let sum = nums[l] + nums[r];
        if sum == target {
            res.push((nums[l], nums[r]));
            // skip duplicate
            while l < r && nums[l] == nums[l + 1] {
                l += 1;
            }
            // skip duplicate
            while l < r && nums[r] == nums[r - 1] {
                r -= 1;
            }
            r -= 1;
            l += 1;
        } else if sum > target {
            r -= 1;
        } else {
            l += 1;
        }
    }

    res
}

fn three_sum_unique_triplets(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    let mut res = Vec::<Vec<i32>>::new();
    for (i, v) in nums.iter().enumerate() {
        // skip duplicate
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        // skip already checked
        let two_sum_result = two_sum(&nums[i + 1..], target - v);

        for tuple in two_sum_result.iter() {
            res.push(vec![tuple.0, tuple.1, *v])
        }
    }

    res
}
