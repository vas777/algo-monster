// Combinatorial Example Problem
// Given a non-negative integer n, find all n-letter words composed by 'a' and 'b', return them in a list of strings in lexicographical order.

// Input: 2
// Output: ["aa", "ab", "ba", "bb"]

// Input: 4
// Output: ["aaaa", "aaab", "aaba", "aabb", "abaa", "abab", "abba", "abbb", "baaa", "baab", "baba", "babb", "bbaa", "bbab", "bbba", "bbbb"]

fn dfs(n: i32, path: &mut String, res_v: &mut Vec<String>) {
    if path.len() == n as usize {
        res_v.push(path.clone());
        return;
    }

    for c in vec!["a", "b"] {
        path.push_str(c);
        dfs(n, path, res_v);
        path.pop();
    }
}

fn letter_combination(n: i32) -> Vec<String> {
    let mut res_v = Vec::<String>::new();
    let mut path = String::new();
    dfs(n, &mut path, &mut res_v);
    return res_v;
}
