// A binary tree is considered balanced if, for every node in the tree, the difference in the height of its left and right subtrees is at most 1.

// In other words, the height of the two subtrees for every node in the tree differs by no more than one.

// The height of a subtree is the number of edges on the longest path from that subtree's root down to any leaf node. (This is distinct from depth, which measures the distance from the tree's root down to a specific node.)

// Note: An empty tree is considered balanced by definition.

// In that case, given a binary tree, determine if it is balanced.

use std::error;
use std::str::FromStr;

pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

fn visit(tree: &Tree<i32>) -> (bool, i32) {
    let Some(root) = tree else {
        return (true, 0);
    };

    let (bl, hl) = visit(&root.left);
    let (br, hr) = visit(&root.right);

    if !(bl && br) {return (false, 0)};
    
    let balanced = (hl - hr).abs() <= 1;

    (balanced, std::cmp::max(hl, hr) + 1)
}

// check if left subree is balanced
// check if right subree is balanced

fn is_balanced(tree: Tree<i32>) -> bool {
    let Some(root) = &tree else {
        return true;
    };

    let (bl, _hl) = visit(&tree);

    return bl;
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
