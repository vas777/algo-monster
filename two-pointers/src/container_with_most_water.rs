use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

// what should make pointer move ?
// two pass will work - not with fixed ends.

fn container_with_most_water(arr: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut l = 0;
    let mut r = arr.len() - 1;
    while l < r {
        let v = (r - l) as i32 * cmp::min(arr[l], arr[r]);
        if max < v {
            max = v;
        }
        // bottleneck rule
        // we could safely abandon smaller wall aka bottleneck
        // for better solution
        if arr[l] < arr[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    max as i32
}

#[cfg(test)]
mod tests {

    use super::*;
    use two_pointers::string_to_vec;

    #[test]
    fn container_with_most_water_works() {
        let test_cases: Vec<(Vec<i32>, i32)> = vec![
            (string_to_vec("1 8 6 2 5 4 8 3 7"), 49),
            (string_to_vec("1 8 6 2 5 4 8 3 7"), 49),
            (string_to_vec("1 1"), 1),
            (string_to_vec("3 2 1 3"), 9),
            (string_to_vec("2 1 2"), 4),
            (string_to_vec("3 1 2 1"), 4),
            (string_to_vec("1 1 1 1 1 1 100 100 1 1 1 1 1 1"), 100),
        ];

        for (input, out) in test_cases {
            assert_eq!(container_with_most_water(input), out);
        }
    }
}
