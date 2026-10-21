// Given a string s, find all ways to partition it so that every substring is a palindrome. Return all possible palindrome partitions of s.

// Examples
// Example 1:
// Input: aab
// Output:

// Copy
//   [
//   ["a","a","b"],
//   ["aa","b"]
//   ]

// two pointers from opposite end
fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut l: usize = 0;
    let mut r = s.len().saturating_sub(1);

    while l < r {
        if bytes[l] != bytes[r] {
            return false;
        }

        r -= 1;
        l += 1;
    }
    return true;
}

pub fn dfs(
    s: &str,
    start_index: usize,
    current_result: &mut Vec<String>,
    res: &mut Vec<Vec<String>>,
) {
    if s.len() == start_index {
        res.push(current_result.clone());
    }

    for end_index in (start_index + 1)..=s.len() {
        if is_palindrome(&s[start_index..end_index]) {
            current_result.push(s[start_index..end_index].to_owned());
            dfs(s, end_index, current_result, res);
            current_result.pop();
        }
    }
}

fn partition(s: String) -> Vec<Vec<String>> {
    let mut res: Vec<Vec<String>> = Vec::new();
    let mut current_result: Vec<String> = Vec::new();
    dfs(s.as_str(), 0, &mut current_result, &mut res);
    res
}
