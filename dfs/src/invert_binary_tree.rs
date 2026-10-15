// Given a binary tree, invert it and return the new value. You may invert it in-place.
//
// To "invert" a binary tree, switch the left subtree and the right subtree, and invert them both. Inverting an empty tree does nothing.

pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

// invert_binary_tree(tree)
// if tree == null
// return
// left = node.left
// right = node.right
// node.left = right
// node.right = left
// invert_binary_tree(node.left)
// invert_binary_tree(node.right)
// return tree

fn invert_binary_tree(tree: Tree<i32>) -> Tree<i32> {
    let Some(mut node) = tree else {
        return None;
    };

    // avoided destructuring by let Node{..} = *node
    // because that  moves fields out of the Box
    // which will lead to unnecessary Box::new allocation

    let left = node.left.take();
    let right = node.right.take();

    node.right = invert_binary_tree(left);
    node.left = invert_binary_tree(right);

    Some(node)
}
