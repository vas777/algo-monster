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
    let mut original_chars: Vec<char> = original.chars().collect();
    let m = original.len();

    let required = check_count.len();
    let mut satisfied = 0;
    let mut window: isize = -1;
    let mut window_len = m + 1;
    let mut l = 0;

    for r in 0..m {
        let right_char = original_chars[r];
        if let Some(original_count) = check_count.get(&right_char) {
            let window_count = window_count.entry(right_char).or_insert(0);
            *window_count += 1;

            if window_count == original_count {
                satisfied += 1;
            }
        }

        while required == satisfied {
            let curr_len = r - l + 1;

            // check if this the better window
            // tie break

            // shrink from left
            // if we are removing char from check
            // decrease satisfied count
        }
    }

    String::new()
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
