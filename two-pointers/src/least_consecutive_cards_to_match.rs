// A bunch of cards is laid out in front of you in a line, where the value of each card ranges from 0 to 10^6. 
// A pair of cards is matching if they have the same number value.

// Given a list of integers cards, your goal is to match a pair of cards, but you can only pick up cards in a consecutive manner. 
// What's the minimum number of cards that you need to pick up to make a pair? If there are no matching pairs, return -1.

// For example, given cards = [3, 4, 2, 3, 4, 7], then picking up [3, 4, 2, 3] makes a pair of 3s and picking up [4, 2, 3, 4] 
// matches two 4s. We need 4 consecutive cards to match a pair of 3s and 4 consecutive cards to match 4s, so you need to pick up at least 4 cards to make a match.


// for r in cards.len
   // add card to window
   // if pair 
    // record distance 
    // shrink from left
    // set value 0 

use std::collections::HashMap;

fn least_consecutive_cards_to_match(cards: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut shortest_distance = usize::MAX;
    let mut pairs: HashMap<i32,i32> = HashMap::new();
    
    for r in 0..cards.len(){
        *pairs.entry(cards[r]).or_insert(0) += 1;
    }

    0
}

