use bfs::{Node, Tree};

use std::collections::VecDeque;

fn binary_tree_right_side_view(root: Tree<i32>) -> Vec<i32> {
    let Some(node) = root.as_ref() else {
        return Vec::new();
    };
    let mut order = VecDeque::<&Node<i32>>::new();
    let mut level = Vec::<i32>::new();

    order.push_back(node);
    let mut node_per_level = 1;

    while let Some(n) = order.pop_front() {
        node_per_level -= 1;

        if let Some(l) = n.left.as_ref() {
            order.push_back(l);
        }

        if let Some(r) = n.right.as_ref() {
            order.push_back(r);
        }

        if node_per_level == 0 {
            level.push(n.val);
            node_per_level = order.len();
        }
    }

    level
}
