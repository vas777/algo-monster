// Flood Fill
// In computer graphics, an uncompressed raster image is presented
// as a matrix of numbers. Each entry of the matrix represents the color
// of a pixel. A flood fill algorithm takes a coordinate r, c and a
// replacement color, and replaces all pixels connected to r, c that
// have the same color (i.e., the pixels connected to the given coordinate
// with same color and all the other pixels connected to the those pixels
// of the same color) with the replacement color. (e.g. MS-Paint's paint bucket tool).

// Input & Output
// Input
// r — row
// c — column
// replacement — replacement color
// image — an 2D array of integers representing the image
// Output
// the replaced image
// Example
// Input
// r = 2
// c = 2
// replacement = 9
// arr = [[0,1,3,4,1],[3,8,8,3,3],[6,7,8,8,3],[12,2,8,9,1],[12,3,1,3,2]]
// Output
// [[0,1,3,4,1],[3,9,9,3,3],[6,7,9,9,3],[12,2,9,9,1],[12,3,1,3,2]]
// Explanation
// From

// Copy
// 0 1 3 4 1
// 3 8 8 3 3
// 6 7 8 8 3
// 12 2 8 9 1
// 12 3 1 3 2
// to

// Copy
// 0 1 3 4 1
// 3 9 9 3 3
// 6 7 9 9 3
// 12 2 9 9 1
// 12 3 1 3 2

use std::collections::{HashSet, VecDeque};

fn get_neighbours(r: i32, c: i32, image: &[Vec<i32>], orig_value: i32) -> Vec<(i32, i32)> {
    let row_len = image.len() as i32;
    let col_len = image[0].len() as i32;
    let mut res = Vec::<(i32, i32)>::new();
    let delta_row = [-1, 0, 1, 0];
    let delta_col = [0, 1, 0, -1];

    for i in 0..4 {
        let row = r + delta_row[i];
        let col = c + delta_col[i];
        if 0 <= row
            && row < row_len
            && 0 <= col
            && col < col_len
            && image[row as usize][col as usize] == orig_value
        {
            res.push((row, col));
        }
    }

    res
}

fn flood_fill(r: i32, c: i32, replacement: i32, image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut res = image.clone();
    let orig_value = image[r as usize][c as usize];
    res[r as usize][c as usize] = replacement;
    queue.push_back((r, c));
    visited.insert((r, c));

    while let Some((node_r, node_c)) = queue.pop_front() {
        for (row_n, col_n) in get_neighbours(node_r, node_c, &image, orig_value) {
            if visited.contains(&(row_n, col_n)) {
                continue;
            }
            res[row_n as usize][col_n as usize] = replacement;
            queue.push_back((row_n, col_n));
            visited.insert((row_n, col_n));
        }
    }

    res
}
