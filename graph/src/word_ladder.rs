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
    if begin == end {
        return 0;
    }
    if begin.is_empty() || begin.len() != end.len() {
        return -1;
    }
    let mut word_list = word_list
        .iter()
        .map(|s| s.as_bytes())
        .collect::<HashSet<&[u8]>>();
    if !word_list.contains(end.as_bytes()) {
        return -1;
    }

    let mut order = VecDeque::<&[u8]>::new();
    let mut mutate = vec![0u8; begin.len()];
    order.push_back(begin.as_bytes());
    word_list.remove(begin.as_bytes());

    let mut distance = 0;
    while !order.is_empty() {
        distance += 1;
        for _ in 0..order.len() {
            if let Some(word) = order.pop_front() {
                mutate.copy_from_slice(word);
                for i in 0..begin.len() {
                    let old = mutate[i];
                    for c in b'a'..=b'z' {
                        if c == old {
                            continue;
                        }
                        mutate[i] = c;
                        if mutate.as_slice() == end.as_bytes() {
                            return distance;
                        }
                        // if it present take it
                        if let Some(nw) = word_list.take(mutate.as_slice()) {
                            order.push_back(nw);
                        }
                    }
                    mutate[i] = old;
                }
            }
        }
    }
    -1
}
