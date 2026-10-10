// Given two strings, original and check, return the shortest substring of
// original that contains every character in check, including duplicates.
// If multiple valid substrings have the same length, return the lexicographically
//  smallest one.

// Parameters
// original: The source string.
// check: The required characters.
// Result
// The minimum valid window in original.
// Examples
// Example 1
// Input: original = "cdbaebaecd", check = "abc"

// Output: baec

// Explanation: Both cdba and baec are valid windows of length 4.
// We return baec because it is lexicographically smaller.

// Constraints
// 1 <= len(check), len(original) <= 10^5
// original and check contain only uppercase and lowercase English letters.
// Characters are case-sensitive.

use std::collections::HashMap;

fn get_minimum_window(original: String, check: String) -> String {
    let mut check_count: HashMap<char, usize> = HashMap::<char, usize>::new();

    for c in check.chars() {
        *check_count.entry(c).or_insert(0) += 1;
    }

    let mut window_count = HashMap::<char, usize>::new();
    let original_chars: Vec<char> = original.chars().collect();
    let m = original.len();

    let mut window: isize = -1;
    let mut window_len = m + 1;
    let mut satisfied = 0;
    let required = check_count.len();
    let mut l = 0;

    for r in 0..m {
        let right_char = original_chars[r];
        if let Some(&original_count) = check_count.get(&right_char) {
            let window_count = window_count.entry(right_char).or_insert(0);
            *window_count += 1;

            if *window_count == original_count {
                satisfied += 1;
            }
        }

        while required == satisfied {
            let curr_window_len = r - l + 1;

            let found_better_window = if curr_window_len < window_len {
                //found better window
                true
            } else if curr_window_len == window_len && window >= 0 {
                // lexicographical comparison
                let curr_candidate = &original_chars[l..l + curr_window_len];
                let existing_candidate =
                    &original_chars[window as usize..window as usize + window_len];
                curr_candidate < existing_candidate
            } else {
                false
            };

            if found_better_window {
                window = l as isize;
                window_len = curr_window_len;
            }

            // shrink from left
            // if we are removing char from check
            // decrease satisfied count
            let left_char = original_chars[l];
            if let Some(&original_count) = check_count.get(&left_char) {
                let window_count = window_count.get_mut(&left_char).expect("must");
                *window_count -= 1;

                if *window_count < original_count {
                    satisfied -= 1;
                }
            }
            l += 1;
        }
    }

    if window >= 0 {
        original_chars[window as usize..window as usize + window_len]
            .iter()
            .collect()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn longest_substring_without_repeating_characters_works() {
        let results: Vec<(&str, &str, &str)> = vec![
            ("cdbaebaecd", "abc", "baec"),
            (
                "aabbababaabaabbaaabbabbabbaabbabaabbabbbbabbaaababbaabb",
                "bababa",
                "aababb",
            ),
        ];

        for (s, c, r) in results {
            assert_eq!(get_minimum_window(s.to_owned(), c.to_owned()), r);
        }
    }
}
