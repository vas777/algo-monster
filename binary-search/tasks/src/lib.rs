fn binary_search(arr: Vec<i32>, target: i32) -> i32 {
    if arr.len() == 0 {
        return -1;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut middle: usize = left + (right - left) / 2;

    while right >= left {
        if arr[middle] == target {
            return middle as i32;
        } else if arr[middle] > target {
            if middle == !0 {
                right = middle;
            } else {
                return -1;
            }
        } else {
            if middle + 1 < arr.len() {
                left = middle + 1;
            } else {
                return -1;
            }
        }
        middle = left + (right - left) / 2;
    }

    return -1;
}

fn find_boundary(arr: Vec<bool>) -> i32 {
    let mut l = 0;
    let mut r = arr.len();
    let mut m = 0;
    while l < r {
        if arr[m] {
            r = m;
        } else {
            l = m + 1;
        }
        m = l + (r - l) / 2;
    }
    if l == arr.len() { -1 } else { l as i32 }
}


fn first_not_smaller(arr: Vec<i32>, target: i32) -> i32 {
    let len = arr.len();
    if len == 0 {
        return -1;
    }

    let mut l = 0;
    let mut r = len-1;
    let mut m = l + (r-l)/2;
    while r>l {
        if arr[m] < target {
            l = m + 1;
        } else if arr[m] >= target {
            r = m;
        }
        m = l + (r-l)/2;
    }

    if m == len-1 && arr[m] < target {
        return -1;
    }
    
    m as i32 
}

fn find_first_occurrence(arr: Vec<i32>, target: i32) -> i32 {
    let len = arr.len();
    if len == 0 {
        return -1;
    }
    let mut l = 0;
    let mut r = arr.len()-1;
    let mut m = l + (r-l)/2;

    while l<r {
        if arr[m] >= target {
            r = m;
        } else if arr[m] < target {
            l = m + 1;
        }
        m = l + (r-l)/2;
    }

    if m == r && arr[m] != target {
        return -1;
    }
    
    m as i32
}


fn square_root_naive(n: i32) -> i32 {

    let mut res = 1;
    res = loop{
      if res*res == n {
          break res;
      } else if res*res > n {
          break res-1;
      } else {
          res +=1;
      } 
    };
    
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_works() {
        let result = binary_search(vec![1, 2, 3, 4, 5, 6, 7, 7, 8, 9], 5);
        assert_eq!(result, 4);
        let result = binary_search(vec![1, 3, 5, 6, 7], 5);
        assert_eq!(result, 2);
        let result = binary_search(vec![1, 2, 3, 4, 5, 6, 7], 0);
        assert_eq!(result, -1);
        let result = binary_search(vec![10, 20], 20);
        assert_eq!(result, 1);
        let result = binary_search(vec![10, 20], 0);
        assert_eq!(result, -1);
        let result = binary_search(vec![10], 0);
        assert_eq!(result, -1);
        let result = binary_search(vec![10], 10);
        assert_eq!(result, 0);
        let result = binary_search(vec![], 10);
        assert_eq!(result, -1);
    }

    #[test]
    fn find_range_works() {
        let result = find_boundary(vec![false, false, true, true, true]);
        assert_eq!(result, 2);
        let result = find_boundary(vec![true]);
        assert_eq!(result, 0);
        let result = find_boundary(vec![false, false, false]);
        assert_eq!(result, -1);
        let result = find_boundary(vec![true, true]);
        assert_eq!(result, 0);
        let result = find_boundary(vec![true, true, true]);
        assert_eq!(result, 0);
        let result = find_boundary(vec![false, true]);
        assert_eq!(result, 1);
        let result = find_boundary(vec![false, false, false, false, false, true]);
        assert_eq!(result, 5);
    }
}
