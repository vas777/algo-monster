// Serializing and Deserializing Binary Tree
// Prerequisite: DFS on Tree
//
// Given a binary tree, write a serialize function that converts the tree into a string and a deserialize function that converts
// a string back to a binary tree. You may serialize the tree into any string representation you want, as long as it can be deserialized properly.

// tried to replicate algomoster serialization logic
// 6 4 3 x x 5 x x 8 x x
// did not expect it to be so nice and recursion friendly

#[derive(Debug)]
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

fn serialize(root: &Option<Box<Node>>) -> String {
    let Some(node) = root else {
        return String::from("x");
    };

    let res = node.val.to_string();

    let l = serialize(&node.left);
    let r = serialize(&node.right);

    res + " " + &l + " " + &r
}

fn des_iter<'a, I>(iter: &mut I) -> Option<Box<Node>>
where
    I: Iterator<Item = &'a str>,
{
    let val = iter.next()?;
    if val == "x" {
        return None;
    }
    let mut node = Node::new(val.parse().unwrap());
    node.left = des_iter(iter);
    node.right = des_iter(iter);
    Some(Box::new(node))
}

fn deserialize(s: &str) -> Option<Box<Node>> {
    des_iter(&mut s.split_whitespace())
}
