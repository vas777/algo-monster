// Find the length of the longest substring of a given string without repeating characters.

// Input: abccabcabcc

// Output: 3

// Explanation: The longest substrings are abc and cab, both of length 3.

// Use the "Sample 1: abccabcabcc" preset in the visualizer below to replay this case.

// Input: aaaabaaa

// Output: 2

// Explanation: ab is the longest substring, with a length of 2.

// Use the "Sample 2: aaaabaaa" preset in the visualizer below to replay this case.

// while string length
// add element to window
// while not_all_unique
// remove all left elements
// record max len

fn longest_substring_without_repeating_characters(s: String) -> i32 {
    let mut l = 0;
    let mut max_window_len = 0;
    // 255 covers ascii symbols
    let mut freq_count: [i32; 255] = [0; 255];
    let bytes = s.as_bytes();

    for r in 0..s.len() {
        freq_count[(bytes[r]) as usize] += 1;

        // only last added could skew our count
        while l < r && freq_count[(bytes[r]) as usize] > 1 {
            freq_count[(bytes[l]) as usize] -= 1;
            l += 1;
        }

        max_window_len = max_window_len.max(r - l + 1);
    }

    max_window_len as i32
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn longest_substring_without_repeating_characters_works() {
        let results: Vec<(&str, i32)> = vec![("abab", 2), ("aaaabaaa", 2), ("abccabcabcc", 3)];

        for (s, r) in results {
            assert_eq!(
                longest_substring_without_repeating_characters(s.to_owned()),
                r
            );
        }
    }
}
