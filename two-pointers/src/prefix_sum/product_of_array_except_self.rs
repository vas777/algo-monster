// Given an integer array nums, return an array answer such that
// answer[i] is equal to the product of all the elements of nums except nums[i].

// Input: [1, 2, 3, 4, 5].
// Output: [120, 60, 40, 30, 24].

// split `product` in two parts / arrays
// left => multiple value to the left of [i]
// right => all values to the right of [i]

// so if we multiply each left and right
// it means that values used at position [i] are `all` to the left of [i]
// and all to the right of [i] but not [i] itself.
// result[i] = arr[0]*arr[1]*arr[i-1]*arr[i+1]

fn product_of_array_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut left = 1;
    for (i, v) in nums.iter().enumerate() {
        res.insert(i, left);
        left *= v;
    }

    let mut right = 1;
    for i in (0..nums.len()).rev() {
        res[i] *= right;
        right *= nums[i];
    }

    res
}

#[cfg(test)]
mod tests {

    use super::*;
    use two_pointers::string_to_vec;

    #[test]
    fn product_of_array_except_self_works() {
        let test_cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
            (string_to_vec("1 2 3 4"), string_to_vec("24 12 8 6")),
            (
                string_to_vec("2 4 6 8 10"),
                string_to_vec("1920 960 640 480 384"),
            ),
        ];

        for (input, out) in test_cases.clone() {
            assert_eq!(product_of_array_except_self(input), out);
        }
    }
}
