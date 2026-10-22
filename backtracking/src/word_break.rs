// Given a string and a list of words, determine if the string can be constructed from concatenating words from the list of words. A word can be used multiple times.

// Input:
// target = "algomonster"
// words = ["algo", "monster"]
// Output: true

// Input:
// target = "aab"
// words = ["a", "c"]
// Output: false

fn dfs(target: &str, words: &[String], start_index: usize, memo: &mut Vec<bool>) -> bool {
    if start_index == target.len() {
        return true;
    }

    if !memo[start_index] {
        return false;
    }

    for w in words.iter() {
        if target[start_index..].starts_with(w) {
            if dfs(target, words, start_index + w.len(), memo) {
                return true;
            }
        }
    }
    memo[start_index] = false;

    false
}

fn word_break(s: String, words: Vec<String>) -> bool {
    // keeps track of position that don't have matching word
    let mut memo = vec![true; s.len()];
    dfs(&s, words.as_slice(), 0, &mut memo)
}
