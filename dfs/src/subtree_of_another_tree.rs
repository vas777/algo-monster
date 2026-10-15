// Given two binary trees root and subRoot, determine if subRoot is a subtree of root.
// A subtree of a binary tree is a tree that consists of a node in the tree and all
// of its descendants. An empty tree is considered a subtree of any tree (including
// another empty tree).

use dfs::{Tree, build_tree};

fn similar(root: &Tree<i32>, sub_root: &Tree<i32>) -> bool {
    if root.is_none() && sub_root.is_none() {
        return true;
    } else if root.is_none() || sub_root.is_none() {
        return false;
    }

    let Some(root_node) = root else {
        return false;
    };
    let Some(sub_root_node) = sub_root else {
        return false;
    };

    let curr = root_node.val == sub_root_node.val;
    let left = similar(&root_node.left, &sub_root_node.left);
    let right = similar(&root_node.right, &sub_root_node.right);

    curr && left && right
}

fn visit(root: &Tree<i32>, sub_root: &Tree<i32>) -> bool {
    let Some(r) = &root else {
        return false;
    };

    let Some(sr) = &sub_root else {
        return true;
    };

    similar(root, sub_root) || visit(&r.left, sub_root) || visit(&r.right, sub_root)
}

fn subtree_of_another_tree(root: Tree<i32>, sub_root: Tree<i32>) -> bool {
    let Some(_) = &sub_root else {
        return true;
    };

    let Some(_) = &root else {
        return false;
    };

    visit(&root, &sub_root)
}
