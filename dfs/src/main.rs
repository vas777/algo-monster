mod bst;
mod invert_binary_tree;
mod is_balanced;
mod subtree_of_another_tree;
mod tree_max_depth;
mod visible_tree_node;
mod ser_des;

fn print_one(c: &str, l: usize) {
    if c.chars().nth(0).unwrap() == '!' {
        print!("{}", c);
        return;
    }

    print_one(&c[l + 1..], l);

    print!("{}", &c[l..l + 1]);
}

fn main() {
    let s = "Hello, world!";
    print_one(s, 0);
}
