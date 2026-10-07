// Given an integer array nums, return an array answer such that 
// answer[i] is equal to the product of all the elements of nums except nums[i].

// Input: [1, 2, 3, 4, 5].
// Output: [120, 60, 40, 30, 24].

// nums[0] = 2 3 4 = 24
// nums[1] = 24 / 2
// nums[2] = 24 / 3
// nums[3] = 24 / 4 

fn product_of_array_except_self(nums: Vec<i32>) -> Vec<i32> {
    let product = nums.iter().product::<i32>();
    let mut ans = Vec::new();
    for v in nums.iter() {
        if *v != 0 {
            ans.push(product/v);
        }else {
            ans.push(0);
        }
    }

    ans
}

