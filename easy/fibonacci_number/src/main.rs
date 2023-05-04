pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 { n } 
        else {
            Self::fib(n-2) + Self::fib(n-1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }
}
