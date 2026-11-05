// On an infinitely large chessboard, a knight is located on [0, 0].

// A knight can move in eight directions.

// Given a destination coordinate [x, y], determine the minimum number of moves from [0, 0] to [x, y].

use std::collections::{HashSet, VecDeque};

fn get_next_moves(r: i32, c: i32, x: i32, y: i32) -> [(i32, i32); 8] {
    let row_len = x + 2;
    let col_len = y + 2;
    let mut res = [(i32::MIN, i32::MIN); 8];
    let delta_row = [2, 1, -1, -2, -2, -1, 1, 2];
    let delta_col = [1, 2, 2, 1, -1, -2, -2, -1];

    for i in 0..delta_col.len() {
        let row = r + delta_row[i];
        let col = c + delta_col[i];
        if -2 <= row && row <= row_len && -2 <= col && col <= col_len {
            res[i] = (row, col);
        }
    }

    res
}

fn get_knight_shortest_path(x: i32, y: i32) -> i32 {
    if x == 0 && y == 0 {
        return 0;
    }
    let (x, y) = (x.abs(), y.abs());
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut distance = 0;
    queue.push_back((0, 0));
    visited.insert((0, 0));
    visited.insert((i32::MIN, i32::MIN));
    let mut nodes_per_level = 1;

    while let Some((n_x, n_y)) = queue.pop_front() {
        nodes_per_level -= 1;
        for (nn_x, nn_y) in get_next_moves(n_x, n_y, x, y).iter() {
            if visited.contains(&(*nn_x, *nn_y)) {
                continue;
            }
            if *nn_x == x && *nn_y == y {
                return distance + 1;
            }
            queue.push_back((*nn_x, *nn_y));
            visited.insert((*nn_x, *nn_y));
        }
        if nodes_per_level == 0 {
            nodes_per_level = queue.len();
            distance += 1;
        }
    }

    distance
}
