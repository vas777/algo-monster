use bfs::{Node, Tree};

use std::collections::VecDeque;

fn zig_zag_traversal(root: Tree<i32>) -> Vec<Vec<i32>> {
    let Some(node) = root.as_ref() else {
        return Vec::new();
    };
    let mut order = VecDeque::<&Node<i32>>::new();
    let mut level = Vec::<i32>::new();
    let mut res = Vec::<Vec<i32>>::new();
    let mut left_first = true;

    order.push_back(node);
    let mut node_per_level = 1;

    while let Some(n) = order.pop_front() {
        level.push(n.val);
        node_per_level -= 1;

        if let Some(l) = n.left.as_ref() {
            order.push_back(l);
        }

        if let Some(r) = n.right.as_ref() {
            order.push_back(r);
        }

        if node_per_level == 0 {
            if left_first {
                left_first = false;
                res.push(level);
            } else {
                level.reverse();
                res.push(level);
                left_first = true;
            }
            node_per_level = order.len();
            level = Vec::<i32>::with_capacity(node_per_level)
        }
    }

    res
}
