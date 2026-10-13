use std::error;
use std::str::FromStr;

pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

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
    fn tree_max_depth_works() {
        let test_cases = vec![(String::from("5 4 3 x x 8 x x 6 x x"), 2)];
        for (t, r) in test_cases.iter() {
            let tree = build_tree(&mut t.split_whitespace()).unwrap();
            assert_eq!(tree_max_depth(tree), *r);
        }
    }
}
