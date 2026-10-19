// Lowest common ancestor (LCA) of two nodes v and w in a tree is the lowest (i.e., deepest) node that
// has both v and w as descendants. We also define each node to be a descendant
// of itself (so if v has a direct connection to w, w is the lowest common ancestor).

// You can assume each node value in the tree is unique and that both target nodes are guaranteed to exist in the tree.

// Given two nodes of a binary tree, find their lowest common ancestor.

use dfs::{Node, Tree};

fn find<'a>(root: &'a Tree<i32>, p: i32, q: i32) -> &'a Tree<i32> {
    let Some(node) = root else {
        return root; // Returns the &'a None already inside root
    };

    if node.val == p || node.val == q {
        return root;
    }

    let l_res = find(&node.left, p, q);
    let r_res = find(&node.right, p, q);

    match (l_res.is_some(), r_res.is_some()) {
        (true, true) => root,
        (true, false) => l_res,
        (false, true) => r_res,
        (false, false) => &node.left, // Any &'a None works, or return l_res (which is None)
    }
}

fn lca<'a>(root: &'a Tree<i32>, node1: &Tree<i32>, node2: &Tree<i32>) -> &'a Tree<i32> {
    let (Some(p), Some(q)) = (node1, node2) else {
        return &None;
    };

    find(root, p.val, q.val)
}
