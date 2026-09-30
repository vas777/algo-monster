// Given an array of integers, move all the 0s to the back of the array while maintaining the relative order of the non-zero elements. Do this in-place using constant auxiliary space.

// Input:

// Copy
// [1, 0, 2, 0, 0, 7]
// Output:

// Copy
// [1, 2, 7, 0, 0, 0]

pub fn move_zeros(nums: &mut Vec<i32>) {
    let mut slow: usize = 0;
    let last = nums.len();

    // iterate over array with fast
    // as soon as we find non zero element
    // swap it with slow pointer and advance it
    // slow tracks correct part
    for fast in 0..last {
        if nums[fast] != 0 {
            nums.swap(slow, fast);
            slow += 1;
        }
    }

    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_zeros_works() {
        let test_cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
            // (vec![0, 1], vec![1, 0]),
            (vec![1, 0, 2, 0, 0, 7], vec![1, 2, 7, 0, 0, 0]),
            (
                vec![1, 0, 0, 2, 0, 0, 7, 0, 0],
                vec![1, 2, 7, 0, 0, 0, 0, 0, 0],
            ),
            (vec![0, 1, 0, 2, 0, 0, 7], vec![1, 2, 7, 0, 0, 0, 0]),
            (vec![3, 1, 0, 1, 3, 8, 9], vec![3, 1, 1, 3, 8, 9, 0]),
        ];

        for (mut input, out) in test_cases {
            move_zeros(&mut input);
            assert_eq!(input.len(), out.len());
            assert_eq!(input, out);
        }
    }
}
