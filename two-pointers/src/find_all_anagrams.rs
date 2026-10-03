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

fn find_all_anagrams(original: String, check: String) -> Vec<i32> {
    let mut check_freq: [i32; 26] = [0; 26];
    let mut str_freq: [i32; 26] = [0; 26];
    let mut ans = Vec::new();

    if check.len() > original.len() {
        return ans;
    }

    for c in check.as_bytes() {
        check_freq[(c - b'a') as usize] += 1;
    }

    let original_bytes = original.as_bytes();
    for i in 0..check.len() {
        str_freq[(original_bytes[i] - b'a') as usize] += 1;
    }

    if str_freq.eq(&check_freq) {
        ans.push(0);
    }

    for right_index in check.len()..original.len() {
        str_freq[(original_bytes[right_index - check.len()] - b'a') as usize] -= 1;
        str_freq[(original_bytes[right_index] - b'a') as usize] += 1;

        if str_freq.eq(&check_freq) {
            ans.push((right_index - check.len() + 1) as i32);
        }
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
            ("cbaebabacd", "abc", vec![0, 6]),
            ("nabanabannaabbaanana", "banana", vec![0, 3, 5, 6, 7, 13]),
            (
                "thequickbrownfoxjumpsoverthelazydog",
                "thelazydogjumpsoverthequickbrownfox",
                vec![0],
            ),
            ("abacbabc", "abc", vec![1, 2, 3, 5]),
            ("afbe", "be", vec![2]),
        ];
        for (s, check, r) in results {
            assert_eq!(find_all_anagrams(s.to_owned(), check.to_owned()), r);
        }
    }
}
