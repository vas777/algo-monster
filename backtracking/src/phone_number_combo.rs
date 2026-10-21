// Given a phone number that contains digits 2-9, find all possible letter combinations the phone number could translate to.

// nput:

// Copy
// "56"
// Output:

// Copy
// ["jm","jn","jo","km","kn","ko","lm","ln","lo"]

use std::{collections::VecDeque, usize};

fn dfs(n: usize, alphabet: &mut VecDeque<&&str>, path: &mut String, res: &mut Vec<String>) {
    if path.len() == n {
        res.push(path.clone());
        return;
    }

    // forgot that pop will modify
    // so when we are back to root
    // it will be empty

    // or could be eliminated  by making keyboard static
    let curr_alphabet = alphabet.pop_front().expect("we must have");
    // println!("curr {:?}",curr_alphabet );

    for value in curr_alphabet.chars() {
        path.push(value);
        dfs(n, alphabet, path, res);
        path.pop();
    }
    // forgot about this
    alphabet.push_front(curr_alphabet);
}

fn letter_combinations_of_phone_number(digits: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut path = String::new();
    let static_alphabet = vec![
        "0", "1", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];
    let mut alphabet = VecDeque::new();
    for digit in digits.chars() {
        let d: usize = digit.to_digit(10).unwrap() as usize;
        alphabet.push_back(static_alphabet.get(d).unwrap());
    }

    // println!("{:?}",alphabet );

    dfs(digits.len(), &mut alphabet, &mut path, &mut res);

    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn letter_combinations_of_phone_number_works() {
        let tests = vec![(
            "56".to_owned(),
            vec!["jm", "jn", "jo", "km", "kn", "ko", "lm", "ln", "lo"],
        )];
        for (t, r) in tests {
            assert_eq!(letter_combinations_of_phone_number(t), r)
        }
    }
}
