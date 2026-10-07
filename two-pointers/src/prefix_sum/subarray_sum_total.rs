use std::collections::HashMap;

// Follow-up: Count all subarrays
// Return the number of subarrays whose sum equals target.


// Given an array of integers nums and an integer k, 
// return the total number of subarrays whose sum equals to k.

// A subarray is a contiguous non-empty sequence of elements within an array.



fn subarray_sum_total(arr: Vec<i32>, target: i32) -> i32 {
    let mut freq = HashMap::<i32, i32>::new();
    // how many times we seen sum 0 appear so far
    freq.insert(0, 1);
    let mut curr_sum = 0;
    let mut res = 0;

    // think of curr_sum a starting line S to `form target`
    // or S==target so it is start and the end
    // (S - target) is potential previous starting line
    // which is exactly target `units` away

    for i in 0..arr.len() {
        curr_sum += arr[i];
        let complement = curr_sum - target;
        // were there a starting line before this one?
        if freq.contains_key(&complement) {
            res += freq.get(&complement).expect("??");
        }
        // save A starting line
        *freq.entry(curr_sum).or_insert(0) += 1;
    }

    res
}

#[cfg(test)]
mod tests {

    use super::*;
    use two_pointers::string_to_vec;

    #[test]
    fn subarray_sum_totalworks() {
        let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
            (string_to_vec("1 1 1"), 1, 3),
            (string_to_vec("1 1 1"), 2, 2),
            (string_to_vec(" 10 5 -5 -20 10"), -10, 3),
            (string_to_vec("1 2 3 6 1 2 1 2 3"), 3, 6),
        ];

        for (input, w, out) in test_cases.clone() {
            assert_eq!(subarray_sum_total(input, w), out);
        }
    }
}
