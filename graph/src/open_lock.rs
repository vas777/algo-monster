// Open the Lock
// You are faced with a 4-wheel lock where each wheel contains the numbers '0' through '9'. Turning a wheel can either increase or decrease its number by one, wrapping around from '9' to '0' or vice versa. A single move involves rotating any one of the wheels by one slot.

// The lock starts with the combination '0000'. However, there are specific combinations termed as "deadends". If the lock lands on any of these deadend combinations, the wheels jam, making it impossible to proceed.

// Your task is to determine the least number of moves needed to reach a given target combination from the starting point without hitting any deadend. If reaching the target is impossible due to deadends, return -1.

// Input & Output
// Input
// target_combo — a string representing the four digit combination to open the lock.
// trapped_combos — a list of strings representing the trapped combinations.
// Output
// An integer representing the number of steps it takes to open the lock, or `-1` if you can't open it without triggering the trap.
// Example
// Input
// target_combo = "0202"
// trapped_combos = ["0201","0101","0102","1212","2002"]
// Output
// 6
// Explanation
// 0000 -> 1000 -> 1100 -> 1200 -> 1201 -> 1202 -> 0202, a total of 6 steps.

// Constraints
// The starting combination (0000) and the final combination is not trapped because that defeats the purpose of the lock.

use std::collections::{HashSet, VecDeque};

fn num_steps(target_combo: String, trapped_combos: Vec<String>) -> i32 {
    let mut order = VecDeque::<[u8; 4]>::new();
    let mut visited = HashSet::<[u8; 4]>::new();

    for c in trapped_combos {
        let b = c - b'0';
        visited.insert(c.as_bytes().try_into().unwrap());
    }

    order.push_back("0000".as_bytes().try_into().unwrap());
    visited.insert("0000".as_bytes().try_into().unwrap());

    let mut distance = 0;
    let target = target_combo.as_bytes().try_into().unwrap();

    while !order.is_empty() {
        distance += 1;
        for _ in 0..order.len() {
            if let Some(combo) = order.pop_front() {
                let next_combos = get_combos(combo);
                for c in next_combos {
                    if c == target {
                        return distance;
                    }
                    if visited.insert(c) {
                        order.push_back(c);
                    }
                }
            }
        }
    }

    0
}
