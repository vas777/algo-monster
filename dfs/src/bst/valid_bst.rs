// A binary search tree is a binary tree with the property that for any node, the value of
// this node is greater than any node in its left subtree and less than any node's value
// its right subtree. In other words, an inorder traversal of a binary search tree yields
// a list of values that is monotonically increasing (strictly increasing).
//
// Given a binary tree, determine whether it is a binary search tree.

use dfs::{Node, Tree};

// left less than node

fn visit(tree: &Tree<i32>, min: i32, max: i32) -> bool {
    let Some(node) = tree else {
        return true;
    };

    if node.val <= min || node.val >= max {
        return false;
    }

    visit(&node.left, min, node.val) && visit(&node.right, node.val, max)
}

fn valid_bst(root: Tree<i32>) -> bool {
    visit(&root, i32::MIN, i32::MAX)
}
