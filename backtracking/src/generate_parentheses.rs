// Given an integer n, generate all strings with n matching parentheses. "matching" parentheses mean

// there is equal number of opening and closing parentheses.
// each opening parenthesis has matching closing parentheses.
// For example, () is a valid string but )( is not a valid string because ) has no matching parenthesis before it and ( has no matching parenthesis after

// Input
// n = 3
// Output
// ((()))
// (()())
// (())()
// ()(())
// ()()()
// Explanation
// There are 5 ways to create a string with 3 matching parentheses.

fn is_valid_parentheses(s: &str) -> bool {
    // let mut stack = Vec::new();
    if s.len() == 1 {
        return true;
    }
    let mut counter = 0;
    for c in s.chars() {
        if c.eq(&'(') {
            // stack.push('(');
            counter += 1;
        } else {
            // let r = stack.pop();
            // let Some(p) = r else { return false };
            // if p.ne(&'(') {
            //     return false;
            // }
            if counter <= 0 {
                return false;
            }
            counter -= 1;
        }
    }

    // stack.is_empty()
    counter == 0
}

fn dfs(n: i32, open_count: i32, close_count: i32, path: &mut String, res: &mut Vec<String>) {
    if n == close_count {
        res.push(path.clone());
        return;
    }

    for p in ["(", ")"] {
        if p == "(" && open_count < n {
            path.push_str(p);
            dfs(n, open_count + 1, close_count, path, res);
            path.pop();
        } else if p == ")" && close_count < open_count {
            path.push_str(p);
            dfs(n, open_count, close_count + 1, path, res);
            path.pop();
        }
    }
}

fn generate_parentheses(n: i32) -> Vec<String> {
    let mut path = String::new();
    let mut res: Vec<String> = Vec::new();
    dfs(n, 0, 0, &mut path, &mut res);
    res
}

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    fn is_valid_parentheses_works() {
        let tests = vec![
            ("((()))", true),
            ("(()())", true),
            ("(())()", true),
            ("(())()()", true),
            ("((((", false),
            ("()))", false),
            (")()()()(", false),
            ("(()", false),
        ];

        for (t, r) in tests {
            println!("{t}");
            assert_eq!(is_valid_parentheses(t), r);
        }
    }
}
