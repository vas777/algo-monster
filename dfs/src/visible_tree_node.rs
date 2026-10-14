use dfs::{Tree, build_tree};

fn visit(root: &Tree<i32>, max: i32) -> i32 {
    let Some(node) = root else {
        return 0;
    };

    let (curr_count, new_max) = if node.val >= max {
        (1, node.val)
    } else {
        (0, max)
    };

    let left_count = visit(&node.left, new_max);
    let right_count = visit(&node.right, new_max);

    curr_count + left_count + right_count
}

fn visible_tree_node(root: Tree<i32>) -> i32 {
    let Some(node) = &root else {
        return 0;
    };

    visit(&root, node.val)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn visible_tree_node_works() {
        let test_cases = vec![(String::from("5 4 3 x x 8 x x 6 x x"), 3)];
        for (t, r) in test_cases.iter() {
            let tree = build_tree(&mut t.split_whitespace()).unwrap();
            assert_eq!(visible_tree_node(tree), *r);
        }
    }
}
