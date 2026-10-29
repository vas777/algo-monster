use bfs::{Node, Tree};

use std::collections::VecDeque;

fn binary_tree_min_depth(root: Tree<i32>) -> i32 {
    let Some(node) = root.as_ref() else {
        return -1;
    };
    let mut order = VecDeque::<&Node<i32>>::new();
    let mut node_per_level = 1;
    let mut level = 0;
    order.push_back(node);

    while let Some(n) = order.pop_front() {
        node_per_level -=1;
        if let Some(l) = n.left.as_ref() {
            order.push_back(l);
        }

        if let Some(r) = n.right.as_ref() {
            order.push_back(r);
        }

        if n.right.is_none() && n.left.is_none() {
            break;
        }

        if node_per_level == 0 {
            node_per_level = order.len();
            level += 1;
        }
    
    }

    level
}