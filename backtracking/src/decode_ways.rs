// We have a message consisting of digits 0-9 to decode.
// Letters are encoded to digits by their positions in the alphabet

// A -> 1
// B -> 2
// C -> 3
// ...
// Y -> 25
// Z -> 26

// Given a non-empty string of digits, how many ways are there to decode it?

// Input: "18"

// Output: 2

// Explanation: "18" can be decoded as "AH" or "R"

// Input: "123"

// Output: 3

// Explanation: "123" can be decoded as "ABC", "LC", "AW"

fn is_letter(digits: &[u8]) -> bool {
    if digits[0] == b'0' {
        return false;
    }

    let res: i32 = if digits.len() == 2 {
        (((digits[0] - b'0') as i32) * 10 + (digits[1] - b'0') as i32)
    } else {
        (digits[0] - b'0') as i32
    };

    res >= 1 && res <= 26
}

fn dfs(digits: &[u8], start_index: usize, memo: &mut Vec<i32>) -> i32 {
    if digits.len() == start_index {
        return 1;
    }

    if memo[start_index] != -1 {
        return memo[start_index];
    }

    let end = std::cmp::min(start_index + 2, digits.len());
    let mut cur = 0;

    for end_index in start_index + 1..=end {
        if is_letter(&digits[start_index..end_index]) {
            cur += dfs(digits, end_index, memo);
        }
    }

    memo[start_index] = cur;

    cur
}

fn decode_ways(digits: String) -> i32 {
    let mut memo = vec![-1; digits.len()];
    dfs(digits.as_bytes(), 0, &mut memo)
}
