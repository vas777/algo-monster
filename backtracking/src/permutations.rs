// Given a string of unique letters, find all of its distinct permutations.

// Permutation means arranging things with an order. For example, permutations of [1, 2] are [1, 2] and [2, 1]. Permutations are best visualized with trees.

// letters = abc
// All distinct permutations
// abc
// acb
// bac
// bca
// cab
// cba

pub fn dfs(
    letters: &str,
    is_latter_used: &mut Vec<bool>,
    current_result: &mut String,
    res: &mut Vec<String>,
) {
    if letters.len() == current_result.len() {
        res.push(current_result.clone());
        return;
    }

    for (i, l) in letters.chars().enumerate() {
        if !is_latter_used[i] {
            is_latter_used[i] = true;
            current_result.push(l);
            dfs(letters, is_latter_used, current_result, res);
            current_result.pop();
            // must undo or else only one permutation will be generated
            // state is shared
            is_latter_used[i] = false;
        }
    }
}

fn permutations(letters: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut current_result: String = String::new();
    let mut is_latter_used = vec![false; letters.len()];
    dfs(
        letters.as_str(),
        &mut is_latter_used,
        &mut current_result,
        &mut res,
    );
    res
}
