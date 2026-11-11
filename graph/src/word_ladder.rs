// Word Ladder is "A puzzle begins with two words, and to solve the puzzle one must find a chain of other words to link the two, in which two adjacent words (that is, words in successive steps) differ by one letter."

// For example: cold → cord → card → ward → warm

// Given a start word, an end word, and a list of dictionary words, determine the minimum number of steps to go from the start word to the end word using only words from the dictionary.

// Input:

// Copy
// start = "cold"
// end = "warm"
// word_list = ["cold", "gold", "cord", "sold", "card", "ward", "warm", "tard"]
// Output:

// Copy
// 4
// Explanation: We can go from cold to warm by going through cold → cord → card → ward → warm

use std::collections::{HashSet, VecDeque};

fn word_ladder(begin: String, end: String, word_list: Vec<String>) -> i32 {
    let mut word_list = word_list
        .iter()
        .map(|s| s.as_str())
        .collect::<HashSet<&str>>();
    let mut order = VecDeque::<String>::new();
    let mut mutate = Vec::<u8>::with_capacity(begin.len());
    order.push_back(begin.clone());

    let mut distance = 0;
    while !order.is_empty() {
        for _ in 0..order.len() {
            if let Some(word) = order.pop_front() {
                mutate = word.as_bytes().to_vec();
                for i in 0..begin.len() {
                    for c in b'a'..b'z' {
                        mutate[i] = c;
                        if mutate == end {
                            return distance;
                        }
                        if word_list.contains(&mutate) {
                            word_list.remove(&mutate);
                            order.push_back(&mutate);
                        }
                    }
                }
            }
        }
        distance += 1;
    }

    distance
}
