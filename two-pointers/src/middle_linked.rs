// Find the middle node of a linked list.

// Input: 0 1 2 3 4

// Output: 2

// If the number of nodes is even, then return the second middle node.

// Input: 0 1 2 3 4 5

// Output: 3

use std::error;
use std::str::FromStr;

type List<T> = Option<Box<Node<T>>>;

#[derive(Clone)]
pub struct Node<T> {
    pub val: T,
    pub next: List<T>,
}

fn middle_of_linked_list(head: List<i32>) -> i32 {
    // slow, fast are mut references to Option<Box<Node<T>>> so &Option<Box<Node<T>>>;
    // but borrow are immutable
    // they could change where they point to but not the value they are point to
    let mut slow = &head;
    let mut fast = &head;

    // THE NOT BAD(?)
    while let Some(curr) = fast {
        let Some(next) = &curr.next else {
            break;
        };
        fast = &next.next;

        let Some(slow_ref) = slow else {
            unreachable!("inconceivable");
        };
        slow = &slow_ref.next;
    }

    // THE UGLY
    // while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
    //     fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    //     slow = &slow.as_ref().unwrap().next;
    // }

    slow.as_ref().unwrap().val
}

fn build_list<'a, T, I>(iter: &mut I) -> Result<List<T>, Box<dyn error::Error>>
where
    T: FromStr,
    I: Iterator<Item = &'a str>,
    <T as FromStr>::Err: 'static + error::Error,
{
    let val = match iter.next() {
        Some(val) => val.parse()?,
        None => return Ok(None),
    };
    let next = build_list(iter)?;
    Ok(Some(Box::new(Node { val, next })))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn middle_of_linked_list_works() {
        let line = "1 2 3".to_owned();
        // let head: List<i32> =
        let test_cases: Vec<(List<i32>, i32)> = vec![
            (
                build_list(&mut "1 2 3".to_owned().split_whitespace()).unwrap(),
                2,
            ),
            (
                build_list(&mut "1 2 3 4 5 6 7 8 9".to_owned().split_whitespace()).unwrap(),
                5,
            ),
            (
                build_list(&mut "1 2".to_owned().split_whitespace()).unwrap(),
                2,
            ),
            (
                build_list(&mut "1".to_owned().split_whitespace()).unwrap(),
                1,
            ),
            (
                build_list(
                    &mut "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19"
                        .to_owned()
                        .split_whitespace(),
                )
                .unwrap(),
                10,
            ),
        ];

        for (list, middle) in test_cases {
            assert_eq!(middle_of_linked_list(list), middle);
        }
    }
}
