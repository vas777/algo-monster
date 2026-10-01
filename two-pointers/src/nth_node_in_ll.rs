// Given the head of a linked list and an integer n, remove the n-th node from the end of the list and return the head of the modified list.

// Input:

// head: the head node of a singly linked list
// n: an integer representing the position from the end (1-indexed)
// Output:

// Return the head of the modified linked list
// Constraints:

// The list has at least 1 node
// n is a valid position from the end (1 <= n <= length of list)
// Examples:

// Example 1:

// Copy
// Input: head = [1, 2, 3, 4], n = 1
// Output: [1, 2, 3]
// Explanation: Remove the 1st node from the end (value 4)
// Example 2:

// Copy
// Input: head = [1, 2, 3, 4], n = 2
// Output: [1, 2, 4]
// Explanation: Remove the 2nd node from the end (value 3)
// Example 3:

// Copy
// Input: head = [1, 2, 3, 4], n = 4
// Output: [2, 3, 4]
// Explanation: Remove the 4th node from the end (the head, value 1)

use std::error;
use std::str::FromStr;

type List<T> = Option<Box<Node<T>>>;

#[derive(Clone)]
pub struct Node<T> {
    pub val: T,
    pub next: List<T>,
}

// move fast ahead of slow nth time
// then in lockstep until fast is at the end
// slow must point at nth
// keep prev to slow so it becomes new head to return
pub fn remove_nth_from_end(head: Option<Box<Node<i32>>>, n: i32) -> Option<Box<Node<i32>>> {
    let mut new_head = Box::new(Node { val: 0, next: head });

    let mut fast = &new_head as *const Box<Node<i32>>;

    unsafe {
        for _ in 0..n {
            if (*fast).next.is_none() {
                return new_head.next;
            }
            fast = (*fast).next.as_ref().unwrap();
        }

        let mut slow = &mut new_head as *mut Box<Node<i32>>;
        while (*fast).next.is_some() {
            fast = (*fast).next.as_ref().unwrap();
            slow = (*slow).next.as_mut().unwrap();
        }

        let mut node_to_remove = (*slow).next.take();
        let new_node = node_to_remove.as_mut().unwrap().next.take();
        (*slow).next = new_node;
        // (*slow).next = node_to_remove.and_then(|n|n.next);
    }

    new_head.next
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

    fn remove_nth_from_end_works() {}
}
