// Given a linked list with potentially a loop, determine whether the linked list from
// the first node contains a cycle in it. For bonus points, do this with constant space.

// Parameters
// nodes: The first node of a linked list with potentially a loop.

// same value means same node -> cycle
// -1 terminates
fn next_index(nodes: &Vec<i32>, idx: i32) -> i32 {
    let next = nodes[idx as usize];
    if next == -1 { idx } else { next }
}

fn has_cycle(nodes: Vec<i32>) -> bool {
    if nodes.is_empty() {
        return false;
    }

    let mut slow_index = next_index(&nodes, 0);
    let mut fast_index = next_index(&nodes, next_index(&nodes, 0));

    while slow_index != fast_index && nodes[fast_index as usize] != -1 {
        slow_index = next_index(&nodes, slow_index);
        fast_index = next_index(&nodes, next_index(&nodes, fast_index));
    }

    nodes[fast_index as usize] != -1
}

#[cfg(test)]
mod tests {

    use super::*;
    use two_pointers::string_to_vec;

    #[test]
    fn has_cycle_works() {
        let test_cases: Vec<(Vec<i32>, bool)> = vec![
            // same value means same node -> cycle
            // -1 terminates
            (string_to_vec("1 6 3 1 2 4 5"), true),
            (string_to_vec("1 6 3 -1 2 4 5"), false),
        ];

        for (input, out) in test_cases {
            assert_eq!(has_cycle(input), out);
        }
    }
}
