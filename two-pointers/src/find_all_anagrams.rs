// Given a string original and a string check, find the starting index of all substrings in original that are anagrams of check. Return the indices in ascending order.

// Parameters
// original: A string
// check: A string
// Result
// A list of integers representing the starting indices of all anagrams of check.
// Examples
// Example 1
// Input: original = "cbaebabacd", check = "abc"

// Output: [0, 6]

// Explanation: original[0:3] = "cba" and original[6:9] = "bac" each contain exactly the same letters as "abc" with different ordering.

// Example 2
// Input: original = "abab", check = "ab"

// Output: [0, 1, 2]

// Explanation: Every length-2 window in "abab" ("ab", "ba", "ab") is an anagram of "ab".

// Constraints
// 1 <= len(original), len(check) <= 10^5
// Each string consists of only lowercase characters in the standard English alphabet.

// check's len - is fixed window size
// count frequencies of check = set one
// could frequencies of window = set two
// if set one == two
// record most left index

use std::{collections::HashMap};

fn find_all_anagrams(original: String, check: String) -> Vec<i32> {
    let mut check_freq: [i32;26] =[0;26] ;
    let mut str_freq:[i32;26] =[0;26];
    let mut ans = Vec::new();

    for c in check.as_bytes() {
        check_freq[(b'a' - c) as usize] += 1;
    }

    let mut l = 0;
    let mut r = check.len() - 1;

    let original_bytes = original.as_bytes();
    for i in 0..r + 1 {
        str_freq[(b'a' - original_bytes[i]) as usize] += 1;
    }

    
    loop {
        
        if str_freq.eq(&check_freq) {
            eprintln!("equal");
            ans.push(l as i32);
        }

        str_freq[(b'a' - original_bytes[l]) as usize] -= 1;
        str_freq[(b'a' - original_bytes[r]) as usize] -= 1;
        l += 1;
        r += 1;
        if r == original.len() {
            break;
        }
        str_freq[(b'a' - original_bytes[l]) as usize] += 1;
        str_freq[(b'a' - original_bytes[r]) as usize] += 1;
    }

    ans
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn find_all_anagrams_works() {
        let results: Vec<(&str, &str, Vec<i32>)> = vec![
            ("abab", "ab", vec![0, 1, 2]),
            ("cbababacd", "abc", vec![0, 6]),
        ];
        for (s, check, r) in results {
            assert_eq!(find_all_anagrams(s.to_owned(), check.to_owned()), r);
        }
    }
}
