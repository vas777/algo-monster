// Determine whether a string is a palindrome, ignoring non-alphanumeric characters and case. Examples:

// Input: Do geese see God? Output: True

// Input: Was it a car or a cat I saw? Output: True

// Input: A brown fox jumping over Output: False

fn is_palindrome(s: &str) -> bool {
    let mut ss = s.to_owned().clone();
    ss.make_ascii_lowercase();
    let len: usize = ss.len() - 1;
    let chars: Vec<char> = ss.chars().collect();
    let mut l = 0;
    let mut r = len;

    while l < r {
        while l < r && !chars[l].is_alphanumeric() {
            l += 1;
        }

        while l < r && !chars[r].is_alphanumeric() {
            r -= 1;
        }

        if chars[l] != chars[r] {
            return false;
        }

        l += 1;
        r -= 1;
    }

    true
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn is_palindrome_works() {
        let results: Vec<(&str, bool)> = vec![
            ("!!@@##", true),
            ("Do geese see God", true),
            ("Was it a car or a cat I saw?", true),
            ("A brown fox jumping over", false),
            ("A man, a plan, a canal: Panama", true),
            ("race a car", false),
            ("a!!!a", true),
        ];
        for (s, r) in results {
            assert_eq!(is_palindrome(s), r);
        }
    }
}
