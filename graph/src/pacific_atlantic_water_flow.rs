// Pacific Atlantic Water Flow
// There is an m x n rectangular island that borders both the Pacific Ocean and the Atlantic Ocean.
// The Pacific Ocean touches the island's left and top edges, and the Atlantic Ocean touches its right and bottom edges.
// You are given an m x n integer matrix heights where heights[r][c] is the height above sea level of the cell at coordinate (r, c).

// Rain water flows from a cell to a neighboring cell (north, south, east, or west) only when the neighbor's
// height is less than or equal to the current cell's height, and any cell adjacent to an ocean can drain into that ocean.
// Return a list of coordinates [r, c] for every cell from which water can flow to both the Pacific and Atlantic oceans.

// Toggle the views above to trace which cells drain to each ocean. The cells that reach both are the answer you must return.

// Input & Output
// Input
// heights — an m x n integer matrix representing the height above sea level of each cell
// Output
// a list of grid coordinates where rain water can flow to both oceans
// Example
// Input
// heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
// Output
// [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
// Example
// Input
// heights = [[2,1],[1,2]]
// Output
// [[0,0],[0,1],[1,0],[1,1]]

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

fn populate_ocean(
    heights: &[Vec<i32>],
    mut init_queue: VecDeque<(usize, usize)>,
    mut reachable: Vec<Vec<bool>>,
) -> Vec<Vec<bool>> {
    while let Some((r, c)) = init_queue.pop_front() {
        let (n, len) = get_neighbours(r, c, &heights);
        for (nr, nc) in n[..len].iter().copied() {
            if !reachable[nr][nc] && heights[r][c] <= heights[nr][nc] {
                reachable[nr][nc] = true;
                init_queue.push_back((nr,nc));
            }
        }
    }

    reachable
}

fn pacific_atlantic_water_flow(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut pacific = VecDeque::<(usize, usize)>::with_capacity(heights.len() * heights[0].len());
    let mut atlantic = VecDeque::<(usize, usize)>::with_capacity(heights.len() * heights[0].len());
    let rows = heights.len();
    let cols = heights[0].len();
    let mut p_reachable = vec![vec![false; cols]; rows];
    let mut a_reachable = vec![vec![false; cols]; rows];
    let mut res: Vec<Vec<i32>> = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            if i == 0 || j == 0 {
                pacific.push_back((i, j));
                p_reachable[i][j] = true;
            }
            if i == rows - 1 || j == cols - 1 {
                atlantic.push_back((i, j));
                a_reachable[i][j] = true;
            }
        }
    }

    let pacific_ocean = populate_ocean(&heights, pacific, p_reachable);
    let atlantic_ocean = populate_ocean(&heights, atlantic, a_reachable);

    for i in 0..rows {
        for j in 0..cols {
            if pacific_ocean[i][j] && atlantic_ocean[i][j] {
                res.push(vec![i as i32, j as i32]);
            }
        }
    }

    res
}
