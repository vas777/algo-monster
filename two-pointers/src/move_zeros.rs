// Given an array of integers, move all the 0s to the back of the array while maintaining the relative order of the non-zero elements. Do this in-place using constant auxiliary space.

// Input:

// Copy
// [1, 0, 2, 0, 0, 7]
// Output:

// Copy
// [1, 2, 7, 0, 0, 0]

pub fn move_zeros(nums: &mut Vec<i32>) {
    let mut num: usize  = 0;
    let mut zeros: usize = 0;

    while num < nums.len() - 1 {
        num += 1;

        while nums[num] != 0 {
            num += 1;
        }

        while nums[zeros] == 0 {
            zeros += 1;
        }

        nums[zeros] = nums[num];
        nums[num] = 0;

        // if nums[slow] == 0 && nums[fast] != 0 {
        //     nums[slow] = nums[fast];
        // }


    }

    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_zeros_works() {
        let test_cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
            (vec![1, 0, 2, 0, 0, 7], vec![1, 2, 7, 0, 0, 0]),
            // 1 2 0 0 0 7
            // 1 2 7 0 0 0
            (vec![1, 0, 0, 2, 0, 0, 7, 0, 0], vec![1, 2, 7, 0, 0, 0, 0]),
            // (vec![0, 1, 0, 2, 0, 0, 7], vec![1, 2, 7, 0, 0, 0]),
        ];

        for (mut input, out) in test_cases {
            move_zeros(&mut input);
            assert_eq!(input.len(), out.len());
            assert_eq!(input, out);
        }
    }
}
