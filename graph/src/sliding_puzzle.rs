// A sliding puzzle consists of a 2 x 3 board with tiles numbered 1 to 5 and one empty space represented by 0. The board is represented as a 2 x 3 matrix. For example:

// The configuration above is represented by [[4, 1, 3], [2, 0, 5]].

// Each move swaps the empty space with an adjacent tile (horizontally or vertically). The goal is to reach the solved state [[1, 2, 3], [4, 5, 0]]:

// Given an initial board configuration, find the minimum number of moves to reach the solved state, or return -1 if impossible.

// Input & Output
// Input
// init_pos — an integer matrix representing the initial position of the puzzle.
// Output
// The number of steps required to solve this puzzle, or `-1` if the puzzle is impossible to solve.
// Example
// Input
// init_pos = [[4, 1, 3], [2, 0, 5]]
// Output
// 5
// Constraints
// The input must be a 2 x 3 integer matrix containing exactly one of each from 0 to 5

use std::collections::{HashSet, VecDeque};

const DIRECTION: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

fn get_fields(
    field_and_zero: ([[i32; 3]; 2], [usize; 2]),
) -> ([([[i32; 3]; 2], [usize; 2]); 3], usize) {
    let mut res = [([[0; 3]; 2], [0; 2]); 3];

    let field = field_and_zero.0;
    // zero cord
    let r = field_and_zero.1[0];
    let c = field_and_zero.1[1];

    let mut len = 0;
    for d in DIRECTION {
        let nrow = r as i32 + d.0;
        let ncol = c as i32 + d.1;
        if nrow >= 0 && nrow < field.len() as i32 && ncol >= 0 && ncol < field[0].len() as i32 {
            let mut new_field = field;
            new_field[r][c] = field[nrow as usize][ncol as usize];
            new_field[nrow as usize][ncol as usize] = 0;
            res[len] = (new_field, [nrow as usize, ncol as usize]);
            len += 1;
        }
    }

    (res, len)
}

fn num_steps(init_pos: Vec<Vec<i32>>) -> i32 {
    let mut init_position: [[i32; 3]; 2] = [[0; 3]; 2];
    init_position[0].copy_from_slice(init_pos[0].as_slice());
    init_position[1].copy_from_slice(init_pos[1].as_slice());

    let target_field = [[1, 2, 3], [4, 5, 0]];

    let mut pos_zero = [0, 0];
    'outer: for r in 0..init_position.len() {
        for c in 0..init_position[r].len() {
            if init_position[r][c] == 0 {
                pos_zero[0] = r;
                pos_zero[1] = c;
                break 'outer;
            }
        }
    }

    let init_field = (init_position, pos_zero);

    let mut order = VecDeque::<([[i32; 3]; 2], [usize; 2])>::new();
    let mut visited = HashSet::<[[i32; 3]; 2]>::new();
    order.push_back(init_field);
    visited.insert(init_position);

    let mut distance = 0;

    while !order.is_empty() {
        for _ in 0..order.len() {
            if let Some(field_and_zero) = order.pop_front() {
                if field_and_zero.0 == target_field {
                    return distance;
                }
                let (next_fields, len) = get_fields(field_and_zero);
                for i in 0..len {
                    if visited.insert(next_fields[i].0) {
                        order.push_back(next_fields[i]);
                    }
                }
            }
        }
        distance += 1;
    }

    -1
}
