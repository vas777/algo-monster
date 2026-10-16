use dfs::{Node, Tree};

// search
// at each node we know where to go

// let Some(node) = tree else {
//  return None
// };

// if val < node.val
// node = visit(node.left, val)
// else if val > node.val
// node = visit(node.right, val)
// else
// error

// let mut node = tree.take();
// if val < node.val
// node.left = new_node
// else if val > node.val
// node.right = new_node

// return Some(node)

fn insert_bst(bst: Tree<i32>, val: i32) -> Tree<i32> {
    let Some(mut node) = bst else {
        return Some(Box::new(Node {
            val: val,
            left: None,
            right: None,
        }));
    };

    if val < node.val {
        let left: Option<Box<Node<i32>>> = node.left.take();
        node.left = insert_bst(left, val);
    } else if val > node.val {
        let right: Option<Box<Node<i32>>> = node.right.take();
        node.right = insert_bst(right, val);
    }

    Some(node)
}
