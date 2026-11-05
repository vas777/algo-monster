// Find the Number of Islands
// Prereq: BFS on Graph

// Given a 2-dimensional matrix of values with 0 and 1,
// count the number of islands of 1. An island consists
// of all 1 value cells and is surrounded by either an edge
// or all 0 cells. Cells can only be adjacent to each other
// horizontally or vertically, not diagonally.

use std::collections::VecDeque;

fn get_neighbours(r: usize, c: usize, grid: &[Vec<i32>], value: i32) -> Vec<(usize, usize)> {
    let row_len = grid.len();
    let col_len = grid[0].len();
    let mut res = Vec::<(usize, usize)>::new();
    let delta_row: [i32; 4] = [-1, 0, 1, 0];
    let delta_col: [i32; 4] = [0, 1, 0, -1];

    for i in 0..4 {
        let row = r as i32 + delta_row[i];
        let col = c as i32 + delta_col[i];
        if 0 <= row
            && row < row_len as i32
            && 0 <= col
            && col < col_len as i32
            && grid[row as usize][col as usize] == value
        {
            res.push((row as usize, col as usize));
        }
    }

    res
}

fn count_number_of_islands(grid: Vec<Vec<i32>>) -> i32 {
    let mut queue = VecDeque::new();
    let mut res = grid.clone();
    let mut island = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if res[i][j] == 0 {
                continue;
            }

            queue.push_back((i, j));
            res[i][j] = 0;

            while let Some((node_r, node_c)) = queue.pop_front() {
                for (row_n, col_n) in get_neighbours(node_r, node_c, &res, 1) {
                    res[row_n as usize][col_n as usize] = 0;
                    queue.push_back((row_n, col_n));
                }
            }
            island += 1;
        }
    }

    island
}
