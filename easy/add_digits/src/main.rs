pub struct Solution;

/**
 * Time Complexity: O(n * k) where n is the number of digits and k the number of recursive calls
 * Space Complexity: O(k), k being the number of recursive calls
 */

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num < 10 {
            return num;
        }

        let mut num = num;

        num = num.to_string().chars() // Convert the number to chars
            .fold( // fold every element into an accumulator and return the final result
                0,
                |acc, c| 
                    acc + c.to_digit(10).unwrap() as i32
                    // to_digit returns a u32 value, so we cast it with `as i32`
            );

        // Finally we call recursively the function until the number < 10
        Solution::add_digits(num)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::add_digits(38), 2);
        assert_eq!(Solution::add_digits(0), 0);
    }
}