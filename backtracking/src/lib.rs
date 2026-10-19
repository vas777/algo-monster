use std::error;
use std::str::FromStr;

pub struct Node<T> {
    pub val: T,
    pub children: Vec<Node<T>>,
}

// this function builds a tree from input; you don't have to modify it
// learn more about how trees are encoded in https://algo.monster/problems/serializing_tree
pub fn build_tree<'a, T, I>(nodes: &mut I) -> Result<Node<T>, Box<dyn error::Error>>
where
    T: FromStr,
    I: Iterator<Item = &'a str>,
    <T as FromStr>::Err: 'static + error::Error,
{
    let val = nodes.next().ok_or("eol")?.parse()?;
    let num = nodes.next().ok_or("eol")?.parse()?;
    let mut children = Vec::new();
    for _ in 0..num {
        children.push(build_tree(nodes)?);
    }
    Ok(Node { val, children })
}
