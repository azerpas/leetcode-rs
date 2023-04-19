pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut acc = 0;
        let mut previous_char = ' ';
        for c in s.chars() {
            acc += match c {
                'I' => 1,
                'V' => if previous_char == 'I' { 3 } else { 5 },
                'X' => if previous_char == 'I' { 8 } else { 10 },
                'L' => if previous_char == 'X' { 30 } else { 50 },
                'C' => if previous_char == 'X' { 80 } else { 100 },
                'D' => if previous_char == 'C' { 300 } else { 500 },
                'M' => if previous_char == 'C' { 800 } else { 1000 },
                _ => 0
            };
            previous_char = c;
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
