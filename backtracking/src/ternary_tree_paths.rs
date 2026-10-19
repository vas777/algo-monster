use backtracking::Node;

fn dfs(root: &Node<i32>, mut path: Vec<String>, res_v: &mut Vec<String>) {
    if root.children.is_empty() {
        path.push(root.val.to_string());
        let mut res = String::new();
        let mut path_i = path.iter();
        res.push_str(path_i.next().expect("must"));

        for s in path_i {
            res.push_str("->");
            res.push_str(&s);
        }
        res_v.push(res);
        return;
    }

    for c in &root.children {
        let mut new_path = path.clone();
        new_path.push(root.val.to_string());
        dfs(c, new_path, res_v);
    }
    return;
}

fn ternary_tree_paths(root: Node<i32>) -> Vec<String> {
    let mut res = Vec::<String>::new();
    dfs(&root, Vec::new(), &mut res);
    return res;
}

#[cfg(test)]
mod test {

    use super::*;
    use backtracking::build_tree;

    #[test]
    fn ternary_tree_paths_works() {
        let tests = vec![(
            "1 3 2 1 5 0 3 0 4 0".to_owned(),
            vec!["1->2->5", "1->3", "1->4"],
        )];
        for (t, r) in tests {
            let ttest = build_tree(&mut t.split_whitespace()).unwrap();
            assert_eq!(ternary_tree_paths(ttest), r)
        }
    }
}
