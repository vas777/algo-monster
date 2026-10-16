use dfs::Tree;

// bst = [6,2,8,0,4,7,9,x,x,3,5]
// p = 2
// q = 8

// output 6 ( root)

fn visit(bst: &Tree<i32>, p: i32, q: i32) -> i32 {
    let Some(node) = &bst else {
        return -1;
    };

    // record the node where search diverged
    // node diverge if the one is bigger and one is smaller

    if node.val > p && node.val > q {
        visit(&node.left, p, q)
    } else if node.val < p && node.val < q {
        visit(&node.right, p, q)
    } else {
        node.val
    }
}

fn iter_lca_on_bst(bst: Tree<i32>, p: i32, q: i32) -> i32 {
    let mut ibst = &bst;

    while let Some(node) = ibst {
        if node.val > p && node.val > q {
            ibst = &node.left;
        } else if node.val < p && node.val < q {
            ibst = &node.right;
        } else {
            return node.val;
        }
    }

    -1
}

fn lca_on_bst(bst: Tree<i32>, p: i32, q: i32) -> i32 {
    visit(&bst, p, q)
}
