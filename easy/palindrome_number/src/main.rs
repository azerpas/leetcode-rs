pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string().chars().rev()
            .collect::<String>() == x.to_string()
    }
}

mod tests {
    use crate::Solution;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
