// Given two arrays representing the preorder and inorder traversals of the same binary tree with unique values, reconstruct the original tree.
//
// Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7].
// 3 is root
// 9 is to the left of 3
// 15,20,7 is the right of 3

// nothing to the left

// 20 is node because it has neighbours to the left and to the right ?

// building  bottom up so we need to find `last node` in preorder list ?

// let's take 3 6 7 pre and 3 6 7 in ;

use dfs::{Node, Tree};

fn construct_binary_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Tree<i32> {
        if preorder.len() == 0 {
        return None;
    }

    if preorder.len() == 1 {
        return Some(Box::new(Node{val: preorder[0], left: None, right:None}));
    }
    
    let mut root = Box::new(Node{val: preorder[0], left: None, right:None});

    let root_val = preorder[0];
    let in_order_root_index = inorder.iter().position(|&r| r == root_val).expect("Must");

    let new_preorder_left = &preorder[1..in_order_root_index+1];
    let new_preorder_right = &preorder[in_order_root_index+1..];

    let new_inorder_left = &inorder[..in_order_root_index];
    let new_inorder_right = &inorder[in_order_root_index+1..];

    // TODO: redo to not use vec 
    root.left = construct_binary_tree(Vec::from(new_preorder_left), Vec::from(new_inorder_left));
    root.right = construct_binary_tree(Vec::from(new_preorder_right), Vec::from(new_inorder_right));

    Some(root)
}