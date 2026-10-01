// Given an array of integers sorted in ascending order, find two numbers that add up to a given target. Return the indices of the two numbers in ascending order. You can assume elements in the array are unique and there is only one solution. Do this in O(n) time and with constant auxiliary space.

// Input:

// arr: a sorted integer array
// target: the target sum we want to reach
// Sample Input: [2, 3, 4, 5, 8, 11, 18], 8

// Sample Output: 1 3

fn two_sum_sorted(arr: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0;
    let mut r = arr.len() - 1;
    let mut res: Vec<i32> = Vec::with_capacity(2);

    while l < r {
        let sum = arr[l] + arr[r];

        if sum == target {
            res.insert(0, l as i32);
            res.insert(1, r as i32);
            break;
        } else if sum > target {
            r -= 1;
        } else {
            l += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {

    use super::*;
    use two_pointers::string_to_vec;

    #[test]
    fn two_sum_sorted_works() {
        let test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
            (string_to_vec("2 3 4 5 8 11 18"), 8, string_to_vec("1 3")),
            (string_to_vec("2 3 5 8 11 15"), 5, string_to_vec("0 1")),
            (
                string_to_vec("1 2 3 10 20 30 50 100"),
                101,
                string_to_vec("0 7"),
            ),
        ];

        for (input, t, out) in test_cases {
            assert_eq!(two_sum_sorted(input, t), out);
        }
    }
}
