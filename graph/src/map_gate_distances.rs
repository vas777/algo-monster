// Walls and Gates / Zombie in Matrix
// You are given an m by n grid of integers representing a map of a dungeon. In this map:

// -1 represents a wall or an obstacle of some kind.
// 0 represents a gate out of the dungeon.
// INF represents empty space.
// For this question, let INF be 2^31 - 1 == 2147483647, which is the max value of the integer type in many programming languages.

// Venturing into the dungeon is very dangerous, so you would like to know how close you are at each point in the dungeon to the closest exit. Given the map of the dungeon, return the same map, but for each empty space, that space is replaced by the number of steps it takes to reach the closest exit. If a space cannot reach the exit, that space remains INF.

// Note that each step, you can move horizontally or vertically, but you cannot move pass a wall or an obstacle.

// Input & Output
// Input
// dungeon_map — a matrix of integer representing the dungeon map.
// Output
// A matrix of integer representing the dungeon map with the addition of distance to an exit for each empty space.
// Example
// Input
// dungeon_map = [
//   [INF, -1, 0, INF],
//   [INF, INF, INF, -1],
//   [INF, -1, INF, -1],
//   [0, -1, INF, INF],
// ]
// Output
// [
//   [3, -1, 0, 1],
//   [2, 2, 1, -1],
//   [1, -1, 2, -1],
//   [0, -1, 3, 4],
// ]
// Constraints
// 1 <= n, m <= 500

use std::collections::VecDeque;

fn get_neighbours(r: usize, c: usize, grid: &[Vec<i32>]) -> ([(usize, usize); 4], usize) {
    let row_len = grid.len();
    let col_len = grid[0].len();
    let mut res: [(usize, usize); 4] = [(0, 0); 4];
    let delta_row: [i32; 4] = [-1, 0, 1, 0];
    let delta_col: [i32; 4] = [0, 1, 0, -1];
    let mut len = 0;
    for i in 0..4 {
        let row = r as i32 + delta_row[i];
        let col = c as i32 + delta_col[i];
        if 0 <= row && row < row_len as i32 && 0 <= col && col < col_len as i32 {
            res[len] = (row as usize, col as usize);
            len += 1;
        }
    }

    (res, len)
}

fn map_gate_distances(mut dungeon_map: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut queue =
        VecDeque::<(usize, usize)>::with_capacity(dungeon_map.len() * dungeon_map[0].len());

    for i in 0..dungeon_map.len() {
        for j in 0..dungeon_map[0].len() {
            if dungeon_map[i][j] == 0 {
                queue.push_back((i, j));
            }
        }
    }

    while let Some((r, c)) = queue.pop_front() {
        let (n, len) = get_neighbours(r, c, &dungeon_map);
        for (nr, nc) in n[..len].iter().copied() {
            if dungeon_map[nr][nc] == i32::MAX {
                dungeon_map[nr][nc] = dungeon_map[r][c] + 1;
                queue.push_back((nr, nc));
            }
        }
    }

    dungeon_map
}
