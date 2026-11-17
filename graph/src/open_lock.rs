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

fn get_combos(combo: [u8; 4]) -> [[u8; 4]; 8] {
    let mut res = [combo; 8];
    // 0000 gives 8 combinations
    // rotate up (+1)
    // 1000, 0100, 0010, 0001
    for e in 0..=3 {
        if res[e][e] == b'9' {
            res[e][e] = b'0';
        } else {
            res[e][e] += 1;
        }
    }

    // rotate down (-1)
    // 9000, 0900, 0090, 0009
    for e in 0..=3 {
        if res[e + 4][e] == b'0' {
            res[e + 4][e] = b'9';
        } else {
            res[e + 4][e] -= 1;
        }
    }

    res
}

fn num_steps(target_combo: String, trapped_combos: Vec<String>) -> i32 {
    let mut order = VecDeque::<[u8; 4]>::new();
    let mut visited = HashSet::<[u8; 4]>::new();

    for c in trapped_combos {
        // deadends are visited
        visited.insert(c.as_bytes().try_into().unwrap());
    }

    order.push_back([b'0'; 4]);
    if !visited.insert([b'0'; 4]) {
        // was in trapped_combos
        return -1;
    }

    let mut distance = 0;
    let target: [u8; 4] = target_combo.as_bytes().try_into().unwrap();
    // if target == [b'0'; 4] {
    //     return 0;
    // }
    while !order.is_empty() {
        // distance += 1 here if we look for the solution 
        // at the `next level` terminates faster
        // but needs check if target is 0000
        for _ in 0..order.len() {
            if let Some(combo) = order.pop_front() {
                if combo == target {
                    return distance;
                }
                let next_combos = get_combos(combo);
                for c in next_combos {
                    // .insert succeeds iff combination
                    // was not inserted before
                    if visited.insert(c) {
                        order.push_back(c);
                    }
                }
            }
        }
        distance += 1;
    }
    -1
}
