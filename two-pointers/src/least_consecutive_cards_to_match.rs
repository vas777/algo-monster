// A bunch of cards is laid out in front of you in a line, where the value of each card ranges from 0 to 10^6.
// A pair of cards is matching if they have the same number value.

// Given a list of integers cards, your goal is to match a pair of cards, but you can only pick up cards in a consecutive manner.
// What's the minimum number of cards that you need to pick up to make a pair? If there are no matching pairs, return -1.

// For example, given cards = [3, 4, 2, 3, 4, 7], then picking up [3, 4, 2, 3] makes a pair of 3s and picking up [4, 2, 3, 4]
// matches two 4s. We need 4 consecutive cards to match a pair of 3s and 4 consecutive cards to match 4s, so you need to pick up at least 4 cards to make a match.

// for r in cards.len
//
//    while add card to window does make a pair
//      record distance
//      shrink from left until no pairs left

use std::collections::HashSet;

fn least_consecutive_cards_to_match(cards: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut shortest_distance = usize::MAX;
    let mut pairs: HashSet<i32> = HashSet::new();

    for r in 0..cards.len() {
        // 1 2 1 1
        // after detecting pair between 2 and 0
        // second 1 it will be reinserted
        // to check while condition and go to for
        while !pairs.insert(cards[r]) {
            shortest_distance = shortest_distance.min(r - l + 1);
            pairs.remove(&cards[l]);
            l += 1;
        }
    }

    shortest_distance as i32
}

#[cfg(test)]
mod test {

    use super::*;
    use two_pointers::string_to_vec;

    #[test]
    fn least_consecutive_cards_to_match_works() {
        let results: Vec<(Vec<i32>, i32)> = vec![
            (string_to_vec("3 4 2 3 4 7"), 4),
            (string_to_vec("1 2 1 1"), 2),
            (string_to_vec("7 7"), 2),
            (string_to_vec("1 2 3 4 5"), -1),
        ];

        for (s, r) in results {
            assert_eq!(least_consecutive_cards_to_match(s), r);
        }
    }
}
