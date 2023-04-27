pub struct Solution;

/**
 * Time Complexity: O(log(n))
 * Space Complexity: O(1)
 * 
 * - When a bulb has been toggled an even number of times, it is on
 * - When a bulb has been toggled an odd number of times, it is off
 * 
 * - Based on the above, we need to find the number of perfect squares based on n
 */

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        f64::sqrt(n as f64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::bulb_switch(3), 1);
        assert_eq!(Solution::bulb_switch(0), 0);
        assert_eq!(Solution::bulb_switch(1), 1);
        assert_eq!(Solution::bulb_switch(4), 2);
        assert_eq!(Solution::bulb_switch(16), 4);
        assert_eq!(Solution::bulb_switch(10), 3);
    }
}
