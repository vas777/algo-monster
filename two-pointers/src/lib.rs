pub fn string_to_vec(arr: &str) -> Vec<i32> {
    return arr
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();
}
