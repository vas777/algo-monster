use std::error;
use std::io;
use std::str::FromStr;

pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}


fn visit(root: Tree<i32>, max: i32) -> i32 {
    let mut root_v = i32::MIN;
    let mut left_v = i32::MIN;
    let mut right_v = i32::MIN;
    let mut left_node = None;
    let mut right_node = None;
    if let Some(root) = root {
        root_v = root.val;
        left_node = root.left;
        right_node = root.right;
    };

    if let Some(left) = left_node {
        left_v = left.val;
    };

    if let Some(right) = right_node {
        right_v = right.val;
    };


    // let max = std::cmp::max(node.val, v2)
    // count nodes
    // let ld = visit(node.left, depth);
    // let rd = visit(node.right, depth );

    0
    
}

fn tree_max_depth(root: Tree<i32>) -> i32 {
    if root.is_none() {
       return 0;
    }

    visit(root, 0)
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
