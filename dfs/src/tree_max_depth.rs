use dfs::{Tree, build_tree};

fn visit(root: Tree<i32>, depth: i32) -> i32 {
    let Some(node) = root else {
        return depth;
    };

    // count nodes
    let ld = visit(node.left, depth);
    let rd = visit(node.right, depth);

    return std::cmp::max(ld, rd) + 1;
}

fn tree_max_depth(root: Tree<i32>) -> i32 {
    let res = visit(root, 0);
    if res == 0 {
        return 0;
    } else {
        return res - 1;
    };
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn tree_max_depth_works() {
        let test_cases = vec![(String::from("5 4 3 x x 8 x x 6 x x"), 2)];
        for (t, r) in test_cases.iter() {
            let tree = build_tree(&mut t.split_whitespace()).unwrap();
            assert_eq!(tree_max_depth(tree), *r);
        }
    }
}
