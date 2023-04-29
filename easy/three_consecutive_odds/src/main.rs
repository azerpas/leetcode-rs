pub struct Solution;

/**
 * Time complexity: O(n)
 * Space complexity: O(1)
 */

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut consecutive = 0;
        for i in 0..arr.len() {
            if arr[i] % 2 == 1 { // odd number
                consecutive += 1;
            } else {
                consecutive = 0;
            }
            if consecutive >= 3 {
                return true; // early return
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::three_consecutive_odds(vec![2,6,4,1]), false);
        assert_eq!(Solution::three_consecutive_odds(vec![1,2,34,3,4,5,7,23,12]), true);
    }
}