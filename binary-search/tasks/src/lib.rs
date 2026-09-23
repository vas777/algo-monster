fn binary_search(arr: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len();
    let mut middle: usize = 0;
    // Your binary search implementation is fine for the “vanilla” problem
    // as long as arr is sorted in nondecreasing order. It uses the common 
    // half-open interval invariant: search in [left, right) with right = arr.len(), 
    // loop while left < right, and shrink with right = mid / left = mid + 1. 
    // That’s consistent and correct.
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
    if len == 0 {
        return -1;
    }

    let mut l = 0;
    let mut r = len - 1;
    let mut m = l + (r - l) / 2;
    while r > l {
        if arr[m] < target {
            l = m + 1;
        } else if arr[m] >= target {
            r = m;
        }
        m = l + (r - l) / 2;
    }

    if m == len - 1 && arr[m] < target {
        return -1;
    }

    m as i32
}

fn find_first_occurrence(arr: Vec<i32>, target: i32) -> i32 {
    let len = arr.len();
    if len == 0 {
        return -1;
    }
    let mut l = 0;
    let mut r = arr.len() - 1;
    let mut m = l + (r - l) / 2;

    while l < r {
        if arr[m] >= target {
            r = m;
        } else if arr[m] < target {
            l = m + 1;
        }
        m = l + (r - l) / 2;
    }

    if m == r && arr[m] != target {
        return -1;
    }

    m as i32
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn find_range_works() {
        let input_result: Vec<(Vec<bool>, i32)> = vec![
            (vec![false, false, true, true, true], 2),
            (vec![true], 0),
            (vec![false, false, false], -1),
            (vec![true, true], 0),
            (vec![true, true, true], 0),
            (vec![false, true], 1),
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
