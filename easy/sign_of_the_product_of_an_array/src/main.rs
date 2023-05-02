pub struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.iter().fold(1, |prod, x| prod * x.signum())
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
        assert_eq!(Solution::array_sign(vec![1, 5, 0, 2, -3]), 0);
        assert_eq!(Solution::array_sign(vec![-1, 1, -1, 1, -1]), -1);
    }
}
