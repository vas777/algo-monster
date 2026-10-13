use std::error;
use std::io;
use std::str::FromStr;

pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

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

// this function builds a tree from input
// learn more about how trees are encoded in https://algo.monster/problems/serializing_tree
fn build_tree<'a, T, I>(nodes: &mut I) -> Result<Tree<T>, Box<dyn error::Error>>
where
    T: FromStr,
    I: Iterator<Item = &'a str>,
    <T as FromStr>::Err: 'static + error::Error,
{
    let val = match nodes.next().ok_or("eol")? {
        "x" => return Ok(None),
        v => v.parse()?,
    };
    let left = build_tree(nodes)?;
    let right = build_tree(nodes)?;
    Ok(Some(Box::new(Node { val, left, right })))
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
