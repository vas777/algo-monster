// Determine whether a string is a palindrome, ignoring non-alphanumeric characters and case. Examples:

// Input: Do geese see God? Output: True

// Input: Was it a car or a cat I saw? Output: True

// Input: A brown fox jumping over Output: False


fn is_palindrome(s: String) -> bool {
    let mut ss = s.clone();
    ss.make_ascii_lowercase();
    let len: usize = ss.len()-1;
    let bytes = ss.as_bytes();
    let mut l = 0;
    let mut r = len;

    while l <= r {
        let mut lb = bytes[l];
        while lb == b' ' {
            l += 1;
            lb = bytes[l]
        }
        
        let mut rb = bytes[r];
        while rb == b' ' {
            r -= 1;
            rb = bytes[r];
        }

        if bytes[l] == bytes[r] {
            l += 1;
            r -= 1;
        } else {
            return false;
        }
    }
     
    true
}

