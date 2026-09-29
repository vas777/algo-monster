// Given a sorted list of numbers with length at least 1, remove duplicates and return the new length. You must do this in-place and without using extra memory.

// Input: [0, 0, 1, 1, 1, 2, 2].

// Output: 3.

// Your function should modify the list in place so that the first three elements become 0, 1, 2. Return 3 because the new length is 3.

fn remove_duplicates(arr: &mut Vec<i32>) -> usize {
    let mut slow = 0;
    let mut fast = 0;

    while fast < arr.len() - 1 {
        fast += 1;
        if arr[fast] != arr[slow] {
            slow += 1;
            arr[slow] = arr[fast];
        }
    }

    slow + 1 as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_duplicates_works() {
        let input_target_result: Vec<(Vec<i32>, usize)> = vec![
            (vec![7], 1),
            (vec![1, 2, 3], 3),
            (vec![0, 0, 1, 1, 1, 2, 2], 3),
            (vec![0, 0, 1, 1, 1, 2, 2, 3], 4),
            (vec![0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3], 4),
            (vec![1, 1, 1, 1, 1, 1, 1, 1], 1),
            (vec![0, 0, 1, 1, 1, 2, 2, 200, 200, 300, 3000], 6),
        ];

        for (i, r) in input_target_result {
            assert_eq!(
                remove_duplicates(&mut i.clone()),
                r,
                "in {:?} result {}",
                i,
                r
            )
        }
    }
}
