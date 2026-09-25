fn binary_search(arr: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len();
    let mut middle: usize = 0;
    // Your binary search implementation is fine for the “vanilla” problem
    // as long as arr is sorted in nondecreasing order. It uses the common
    // half-open interval invariant: search in [left, right) with right = arr.len(),
    // loop while left < right, and shrink with right = mid / left = mid + 1.
    // That’s consistent and correct.

    // invariant if target exists within [0, len)
    while left < right {
        middle = left + (right - left) / 2;
        if arr[middle] == target {
            return middle as i32;
        } else if arr[middle] < target {
            left = middle + 1;
        } else {
            right = middle;
        }
    }

    return -1;
}

fn find_boundary(arr: Vec<bool>) -> i32 {
    let mut l = 0;
    let mut r = arr.len();
    let mut m = 0;
    while l < r {
        if arr[m] {
            r = m;
        } else {
            l = m + 1;
        }
        m = l + (r - l) / 2;
    }
    if l == arr.len() { -1 } else { l as i32 }
}

fn first_not_smaller(arr: Vec<i32>, target: i32) -> i32 {
    let len = arr.len();
    let mut l = 0;
    let mut r = arr.len();
    let mut m = l + (r - l) / 2;
    while l < r {
        if arr[m] < target {
            l = m + 1;
        } else if arr[m] >= target {
            r = m;
        }
        m = l + (r - l) / 2;
    }

    if l == len {
        return -1;
    }

    m as i32
}

fn find_first_occurrence(arr: Vec<i32>, target: i32) -> i32 {
    let len = arr.len();
    let mut l = 0;
    let mut r = arr.len();
    let mut m = l + (r - l) / 2;
    let mut res: i32 = -1;
    while l < r {
        if arr[m] == target {
            res = m as i32;
            r = m;
        } else if arr[m] > target {
            r = m;
        } else if arr[m] < target {
            l = m + 1;
        }
        m = l + (r - l) / 2;
    }

    res as i32
}

fn square_root_naive(n: i32) -> i32 {
    let mut res = 1;
    res = loop {
        if res * res == n {
            break res;
        } else if res * res > n {
            break res - 1;
        } else {
            res += 1;
        }
    };

    return res;
}

fn square_root(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    let mut l = 0;
    // sqrt(n) <= n/2
    // n^2 > n*2
    let mut r = n / 2 + 1;
    let mut m = l + (r - l) / 2;
    let mut res = 0;

    while l < r {
        if m * m <= n {
            res = m;
            l = m + 1;
        } else if m * m > n {
            r = m - 1;
        }
        m = l + (r - l) / 2;
    }

    return res;
}

// Find Minimum in Rotated Sorted Array
// A sorted array of unique integers was rotated at an unknown pivot.
// For example, [10, 20, 30, 40, 50] becomes [30, 40, 50, 10, 20].
// Find the index of the minimum element in this array.
// Input: [30, 40, 50, 10, 20]
// Output: 3
// Explanation: The smallest element is 10, and its index is 3.
// Input: [3, 5, 7, 11, 13, 17, 19, 2]
// Output: 7
// Explanation: The smallest element is 2, and its index is 7.

// after rotation array has 2 parts `bigger and smaller`
// any element in bigger array is bigger than any element in smaller array
//
// so arr[m] <= arr[arr.len()-1]
// if it is true then
// it means that element at m belongs to smaller part
// and the drop is either m or to its left
// therefore we could shrink search to r = m ;
// in reverse it means that last belongs to the smaller part
// and drop is to the right of m so shrink l = m + 1;

fn find_min_rotated(arr: Vec<i32>) -> i32 {
    if arr.is_empty() {
        return -1;
    }
    let mut l = 0;
    let mut r = arr.len();

    // terminates l == r
    while l < r {
        let mut m = l + (r - l) / 2;
        if arr[m] <= arr[arr.len() - 1] {
            // this m becomes higher bound
            // pivot only could be below this m
            r = m;
        } else {
            // m is bigger and pivot should be ahead.
            l = m + 1;
        }
    }

    l as i32
}

// A mountain array is defined as an array that

// has at least 3 elements
// has an element with the largest value called "peak", with index k. The array elements strictly increase from the first element to A[k], and then strictly decrease from A[k + 1] to the last element of the array. Thus creating a "mountain" of numbers.
// That is, given A[0]<...<A[k-1]<A[k]>A[k+1]>...>A[n-1], we need to find the index k. Note that the peak element is neither the first nor the lastIndex of the array.

// Find the index of the peak element. Assume there is only one peak element.

// Input: 0 1 2 3 2 1 0

// Output: 3

// Explanation: The largest element is 3, and its index is 3.

fn peak_of_mountain_array(arr: Vec<i32>) -> i32 {
    if arr.len() < 3 {
        return -1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peak_of_mountain_array_works() {
        let input_target_result: Vec<(Vec<i32>, i32)> = vec![(vec![0, 1, 2, 3, 2, 1, 0], 3)];

        for (i, r) in input_target_result {
            assert_eq!(
                peak_of_mountain_array(i.clone()),
                r,
                "in {:?} result {}",
                i,
                r
            )
        }
    }

    #[test]
    fn find_min_rotated_works() {
        let input_target_result: Vec<(Vec<i32>, i32)> = vec![
            (vec![30, 40, 50, 10, 20], 3),
            (vec![0, 1, 2, 3, 4, 5], 0),
            (vec![0], 0),
            (vec![1, 2, 3, 5, 8, 0], 5),
            (vec![5, 1, 2, 3, 4], 1),
            (vec![2, 1], 1),
            (vec![1, 2], 0),
            (
                vec![
                    100, 200, 300, 400, 500, 1, 10, 20, 30, 40, 50, 60, 70, 80, 90,
                ],
                5,
            ),
            (vec![-5, -4, -3, -2, -1, -10, -9, -8, -7, -6], 5),
            (vec![7, 8, 9, 10, 1, 2, 3, 4, 5, 6], 4),
            (
                vec![
                    10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170,
                    180, 190, 200,
                ],
                0,
            ),
            (
                vec![
                    200, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160,
                    170, 180, 190,
                ],
                1,
            ),
            (vec![3, 4, 5, 1, 2], 3),
            (vec![4, 5, 6, 7, 0, 1, 2], 4),
            (vec![11, 13, 15, 17, 19, 21, 23, 25, 1, 3, 5, 7, 9], 8),
        ];

        for (i, r) in input_target_result {
            assert_eq!(find_min_rotated(i.clone()), r, "in {:?} result {}", i, r)
        }
    }

    #[test]
    fn binary_search_works() {
        let input_target_result: Vec<(Vec<i32>, i32, i32)> = vec![
            (vec![1, 2, 3, 4, 5, 6, 7, 7, 8, 9], 4, 3),
            (vec![1, 2, 3, 4, 5, 6, 7, 7, 8, 9], 9, 9),
            (vec![1, 2, 3, 4, 5, 6, 7, 7, 8, 9], 1, 0),
            (vec![1, 2, 3, 4, 5, 6, 7], 0, -1),
            (vec![1, 3, 5, 6, 7], 5, 2),
            (vec![10, 20], 20, 1),
            (vec![10, 20], 0, -1),
            (vec![10], 0, -1),
            (vec![10], 10, 0),
            (vec![], 0, -1),
        ];

        for (i, t, r) in input_target_result {
            assert_eq!(
                binary_search(i.clone(), t),
                r,
                "in {:?} target {} result {}",
                i,
                t,
                r
            )
        }
    }
    #[test]
    fn first_not_smaller_works() {
        let input_target_result: Vec<(Vec<i32>, i32, i32)> = vec![
            (vec![1, 2, 3, 3, 5, 6, 7, 7, 8, 9], 4, 4),
            (vec![1, 2, 3, 4, 5, 6, 7, 7, 8, 10], 9, 9),
            (vec![2, 2, 3, 4, 5, 6, 7, 7, 8, 9], 1, 0),
            (vec![2, 2, 3, 4, 5, 6, 7], 1, 0),
            (vec![1, 3, 4, 6, 7], 5, 3),
            (vec![10, 20], 20, 1),
            (vec![10, 20], 0, 0),
            (vec![10], 0, 0),
            (vec![10], 10, 0),
            (vec![], 0, -1),
        ];

        for (i, t, r) in input_target_result {
            assert_eq!(
                first_not_smaller(i.clone(), t),
                r,
                "in {:?} target {} result {}",
                i,
                t,
                r
            )
        }
    }

    #[test]
    fn find_boundary_works() {
        let input_result: Vec<(Vec<bool>, i32)> = vec![
            (vec![false, false, true, true, true], 2),
            (vec![true], 0),
            (vec![true, true], 0),
            (vec![true, true, true], 0),
            (vec![false, true], 1),
            (vec![false], -1),
            (vec![false, false, false], -1),
            (vec![false, false, false, false, false, true], 5),
        ];

        for (i, r) in input_result {
            assert_eq!(find_boundary(i), r)
        }
    }

    #[test]
    fn square_root_naive_works() {
        let input_result: Vec<(i32, i32)> = vec![
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 2),
            (8, 2),
            (10, 3),
            (24, 4),
            (12, 3),
            (1001, 31),
            (1001, 31),
            (100001, 316),
        ];

        for (i, r) in input_result {
            assert_eq!(square_root_naive(i), r)
        }
    }

    #[test]
    fn square_root_works() {
        let input_result: Vec<(i32, i32)> = vec![
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 2),
            (8, 2),
            (10, 3),
            (12, 3),
            (24, 4),
            (1001, 31),
            (100001, 316),
        ];

        for (i, r) in input_result {
            assert_eq!(square_root(i), r, "input {} result {}", i, r)
        }
    }
}
